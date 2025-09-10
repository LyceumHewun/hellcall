use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Ok, Result};
use cpal::traits::{DeviceTrait, HostTrait};
use log::{debug, info};
use std::io::{BufReader, Read};
use std::process::{Child, Command, Stdio};
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

        self.is_finalized.store(false, Ordering::Release);

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
        if is_speech {
            *self.silence_start.lock().unwrap() = None;
            self.is_speaking.store(true, Ordering::Release);
        } else if self.is_speaking.load(Ordering::Acquire) {
            let now = std::time::Instant::now();
            // 没有检测到语音，但之前处于说话状态
            if let Some(silence_start) = *self.silence_start.lock().unwrap() {
                // 检查静音持续时间是否超过阈值
                if now.duration_since(silence_start).as_millis()
                    > self.config.vad_silence_duration as u128
                {
                    self.is_speaking.store(false, Ordering::Release);
                    *self.silence_start.lock().unwrap() = None;
                    self.is_finalized.store(true, Ordering::Release);
                }
            } else {
                // 开始静音计时
                *self.silence_start.lock().unwrap() = Some(now);
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
    device_name: String,
    child_process: Option<Child>,
}

impl AudioBufferProcessor {
    pub fn new(recognizer: AudioRecognizer) -> Result<Self> {
        // get default input device name
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("Failed to get default input device")?;
        let device_name = device.name().context("Failed to get device name")?;

        info!("default input device name: {}", &device_name);

        Ok(Self {
            recognizer: Arc::new(Mutex::new(recognizer)),
            device_name,
            child_process: None,
        })
    }

    pub fn start(&mut self, on_result: Box<dyn Fn(RecognitionResult) + Send>) -> Result<()> {
        if self.is_start() {
            self.stop()?;
        }

        let filter = ["highpass=f=100", "lowpass=f=8000"].join(",");

        let child = Command::new("ffmpeg")
            .args(&[
                "-hide_banner",
                "-loglevel", "error",
                "-fflags", "nobuffer",
                "-flags", "low_delay",
                "-flush_packets", "1",
                "-avioflags", "direct",
                "-f",
                #[cfg(target_os = "windows")]
                "dshow",
                #[cfg(target_os = "linux")]
                "pulse",
                #[cfg(target_os = "macos")]
                "avfoundation",
                "-i", format!("audio={}", &self.device_name).as_str(),
                "-ac", "1",
                "-ar", "16000",
                "-af", &filter,
                "-f", "s16le",
                "-",
            ])
            .stdout(Stdio::piped())
            .spawn()?;

        self.child_process = Some(child);

        let stdout = self.child_process.as_mut().unwrap().stdout.take().unwrap();
        let mut reader = BufReader::new(stdout);

        let recognizer = self.recognizer.lock().unwrap();
        let size = (recognizer.config.chunk_time * VOSK_SAMPLE_RATE * 2.0) as usize;
        let recognizer_ref = Arc::clone(&self.recognizer);

        // new thread
        std::thread::spawn(move || -> Result<()> {
            // 固定采样率 16kHz
            let mut vad = Vad::new_with_rate_and_mode(SampleRate::Rate16kHz, VadMode::Aggressive);
            let mut buffer = vec![0u8; size];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }

                // 转成 i16
                let pcm: Vec<i16> = buffer[..n]
                    .chunks_exact(2)
                    .map(|b| i16::from_le_bytes([b[0], b[1]]))
                    .collect();

                // 处理
                let mut recognizer = recognizer_ref
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Failed to lock recognizer {}", e))?;
                recognizer.detect_speech(&pcm, &mut vad)?;
                let _ = recognizer.process_audio_chunk(&pcm)?;
                if let Some(result) = recognizer.finalize()? {
                    on_result(result);
                }
            }
            Ok(())
        });

        Ok(())
    }

    pub fn is_start(&self) -> bool {
        self.child_process.is_some()
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child_process.take() {
            child.kill().context("Failed to kill child process")?;
            child.wait().context("Failed to wait for child process")?;
        }
        Ok(())
    }
}
