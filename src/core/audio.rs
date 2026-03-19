use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{debug, info};
use vosk::{Model, Recognizer};
use webrtc_vad::{SampleRate, Vad, VadMode};

static VOSK_SAMPLE_RATE: f32 = 16000.0;

#[derive(Debug, Clone)]
pub struct AudioRecognizerConfig {
    /// 音频识别的时间段 (秒)
    pub chunk_time: f32,
    /// 音频识别的语法字典
    pub grammar: Vec<String>,
    /// 语音结束后的静音持续时间 (ms)
    pub vad_silence_duration: u64,
}

impl Default for AudioRecognizerConfig {
    fn default() -> Self {
        Self {
            chunk_time: 0.2,
            grammar: Vec::new(),
            vad_silence_duration: 500,
        }
    }
}

impl AudioRecognizerConfig {
    pub fn set_grammar(&mut self, grammar: Vec<String>) {
        self.grammar = grammar;
    }
}

#[derive(Debug, Clone)]
pub struct RecognitionResult {
    pub text: String,
    pub is_partial: bool,
}

pub struct AudioRecognizer {
    model: Arc<Model>,
    recognizer: Recognizer,
    config: AudioRecognizerConfig,
    is_speaking: AtomicBool,
    silence_start: Mutex<Option<std::time::Instant>>,
    is_finalized: AtomicBool,
}

impl Clone for AudioRecognizer {
    fn clone(&self) -> Self {
        let recognizer =
            Recognizer::new_with_grammar(&self.model, VOSK_SAMPLE_RATE, &self.config.grammar)
                .expect("Failed to create Vosk recognizer");

        Self {
            model: self.model.clone(),
            recognizer,
            config: self.config.clone(),
            is_speaking: AtomicBool::new(self.is_speaking.load(Ordering::Acquire)),
            silence_start: Mutex::new(self.silence_start.lock().unwrap().clone()),
            is_finalized: AtomicBool::new(self.is_finalized.load(Ordering::Acquire)),
        }
    }
}

impl AudioRecognizer {
    pub fn new(model_path: &str, config: AudioRecognizerConfig) -> Result<Self> {
        let model = Model::new(model_path)
            .with_context(|| format!("Failed to load Vosk model from {}", model_path))?;
        let recognizer = Recognizer::new_with_grammar(&model, VOSK_SAMPLE_RATE, &config.grammar)
            .context("Failed to create Vosk recognizer")?;

        Ok(Self {
            model: Arc::new(model),
            recognizer,
            config,
            is_speaking: AtomicBool::new(false),
            silence_start: Mutex::new(None),
            is_finalized: AtomicBool::new(false),
        })
    }

    pub fn process_audio_chunk(
        &mut self,
        audio_chunk: &[i16],
    ) -> Result<Option<RecognitionResult>> {
        if self.is_speaking.load(Ordering::Acquire) {
            self.recognizer
                .accept_waveform(audio_chunk)
                .context("Failed to accept waveform")?;
            let result = self.recognizer.partial_result();
            let result = RecognitionResult {
                text: result.partial.to_string(),
                is_partial: true,
            };

            debug!("partial result: {:?}", result);

            return Ok(Some(result));
        }

        Ok(None)
    }

    pub fn finalize(&mut self) -> Result<Option<RecognitionResult>> {
        if !self.is_finalized.load(Ordering::Acquire) {
            return Ok(None);
        }

        let result = self.recognizer.final_result();
        let recognition_result = RecognitionResult {
            text: result
                .single()
                .context("Failed to get final result")?
                .text
                .to_string(),
            is_partial: false,
        };

        self.reset();

        debug!("final result: {:?}", recognition_result);

        Ok(Some(recognition_result))
    }

    /// 检测语音活动
    pub fn detect_speech(&mut self, audio_chunk: &[i16], vad: &mut Vad) -> Result<()> {
        if audio_chunk.is_empty() {
            return Ok(());
        }

        // 切分帧
        // 采样率 * 每帧时间(ms) / 1000
        let samples_per_frame = VOSK_SAMPLE_RATE as usize * 20 / 1000;

        // 连续帧
        let mut active_frames = 0;
        let mut non_active_frames = 0;
        for frame in audio_chunk.chunks_exact(samples_per_frame) {
            let is_active = vad
                .is_voice_segment(frame)
                .map_err(|e| anyhow::anyhow!("Failed to detect speech: {:?}", e))?;

            if is_active {
                active_frames += 1;
                non_active_frames = 0;
            } else {
                active_frames = 0;
                non_active_frames += 1;
            }

            // 连续 3 帧为活动状态，认为是语音
            if active_frames > 3 {
                self.update_speech_state(true);
            }

            // 连续 5 帧为非活动状态，认为是静音
            if non_active_frames > 5 {
                self.update_speech_state(false);
            }
        }
        Ok(())
    }

    /// 更新语音状态
    fn update_speech_state(&mut self, is_speech: bool) {
        let mut silence_start = self.silence_start.lock().unwrap();
        if is_speech {
            *silence_start = None;
            self.is_speaking.store(true, Ordering::Release);
        } else if self.is_speaking.load(Ordering::Acquire) {
            let now = std::time::Instant::now();
            // 没有检测到语音，但之前处于说话状态
            if let Some(start) = *silence_start {
                // 检查静音持续时间是否超过阈值
                if now.duration_since(start).as_millis() > self.config.vad_silence_duration as u128
                {
                    self.is_speaking.store(false, Ordering::Release);
                    *silence_start = None;
                    self.is_finalized.store(true, Ordering::Release);
                }
            } else {
                // 开始静音计时
                *silence_start = Some(now);
            }
        }
    }

    pub fn reset(&mut self) {
        self.recognizer.reset();
        self.is_speaking.store(false, Ordering::Release);
        *self.silence_start.lock().unwrap() = None;
        self.is_finalized.store(false, Ordering::Release);
    }
}

pub struct AudioBufferProcessor {
    recognizer: Arc<Mutex<AudioRecognizer>>,
    input_device_name: String,
    stream: Option<cpal::Stream>,
    thread_handle: Option<JoinHandle<Result<()>>>,
}

impl AudioBufferProcessor {
    pub fn new(recognizer: AudioRecognizer) -> Result<Self> {
        // get default input device name
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("Failed to get default input device")?;
        let input_device_name = device.name().context("Failed to get device name")?;

        info!("default input device name: {}", &input_device_name);

        Ok(Self {
            recognizer: Arc::new(Mutex::new(recognizer)),
            input_device_name,
            stream: None,
            thread_handle: None,
        })
    }

    pub fn new_with_input_device_name(
        recognizer: AudioRecognizer,
        input_device_name: String,
    ) -> Result<Self> {
        Ok(Self {
            recognizer: Arc::new(Mutex::new(recognizer)),
            input_device_name,
            stream: None,
            thread_handle: None,
        })
    }

    pub fn start(&mut self, on_result: Box<dyn Fn(RecognitionResult) + Send>) -> Result<()> {
        if self.is_start() {
            self.stop()?;
        }

        let host = cpal::default_host();
        let mut target_device = None;
        for device in host.input_devices()? {
            if let std::result::Result::Ok(name) = device.name() {
                if name == self.input_device_name {
                    target_device = Some(device);
                    break;
                }
            }
        }
        let device = target_device
            .or_else(|| host.default_input_device())
            .context("Failed to find input device")?;

        let config = device
            .default_input_config()
            .context("Failed to get default input config")?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();
        let sample_format = config.sample_format();

        let (tx, rx) = std::sync::mpsc::channel::<Vec<i16>>();

        let error_callback = |err| log::error!("an error occurred on stream: {}", err);

        let tx_clone = tx.clone();

        fn process_audio_chunk<T: Copy>(
            data: &[T],
            channels: u16,
            sample_rate: u32,
            to_f32: impl Fn(T) -> f32,
        ) -> Vec<i16> {
            let mut output = Vec::new();
            let ratio = sample_rate as f32 / 16000.0;
            let frames = data.len() / channels as usize;
            let out_len = (frames as f32 / ratio).ceil() as usize;

            for i in 0..out_len {
                let in_frame = (i as f32 * ratio) as usize;
                if in_frame >= frames {
                    break;
                }
                let in_idx = in_frame * channels as usize;

                let mut sum = 0.0;
                for c in 0..channels as usize {
                    sum += to_f32(data[in_idx + c]);
                }
                let mono_sample = sum / channels as f32;

                let sample_i16 = (mono_sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                output.push(sample_i16);
            }
            output
        }

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    let _ = tx_clone.send(process_audio_chunk(data, channels, sample_rate, |x| x));
                },
                error_callback,
                None,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| {
                    let _ = tx_clone.send(process_audio_chunk(data, channels, sample_rate, |x| {
                        x as f32 / i16::MAX as f32
                    }));
                },
                error_callback,
                None,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &_| {
                    let _ = tx_clone.send(process_audio_chunk(data, channels, sample_rate, |x| {
                        (x as f32 - u16::MAX as f32 / 2.0) / (u16::MAX as f32 / 2.0)
                    }));
                },
                error_callback,
                None,
            )?,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported sample format {:?}",
                    sample_format
                ));
            }
        };

        stream.play()?;
        self.stream = Some(stream);

        let recognizer = self.recognizer.lock().unwrap();
        let chunk_time = recognizer.config.chunk_time;
        drop(recognizer);

        let samples_per_chunk = (chunk_time * VOSK_SAMPLE_RATE) as usize;
        let recognizer_ref = Arc::clone(&self.recognizer);

        let handle = std::thread::spawn(move || -> Result<()> {
            let mut vad = Vad::new_with_rate_and_mode(SampleRate::Rate16kHz, VadMode::Aggressive);
            let mut buffer: Vec<i16> = Vec::new();

            for mut pcm in rx.iter() {
                buffer.append(&mut pcm);

                while buffer.len() >= samples_per_chunk {
                    let chunk: Vec<i16> = buffer.drain(..samples_per_chunk).collect();

                    let mut recognizer = recognizer_ref
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Failed to lock recognizer {}", e))?;

                    recognizer.detect_speech(&chunk, &mut vad)?;
                    let _ = recognizer.process_audio_chunk(&chunk)?;
                    if let Some(result) = recognizer.finalize()? {
                        on_result(result);
                    }
                }
            }
            Ok(())
        });
        self.thread_handle = Some(handle);

        Ok(())
    }

    pub fn is_start(&self) -> bool {
        self.stream.is_some()
    }

    pub fn stop(&mut self) -> Result<()> {
        // Drop the stream first: this drops the tx_clone inside the stream callback,
        // which closes the mpsc channel and causes rx.iter() in the thread to end.
        self.stream = None;
        // Now join the processing thread to ensure it has fully exited.
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
        Ok(())
    }
}

impl Drop for AudioBufferProcessor {
    fn drop(&mut self) {
        // Ensure stream and thread are cleaned up even if stop() was not called explicitly.
        let _ = self.stop();
    }
}
