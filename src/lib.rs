pub mod config;
pub mod core;
pub mod utils;

pub use config::Config;

use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait};
use log::{info, warn};
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

use crate::core::audio::*;
use crate::core::command::*;
use crate::core::keypress::*;
use crate::core::matcher::*;
use crate::core::speaker::*;
use crate::utils::*;

static AUDIO_DIR: &str = "audio";

pub struct HellcallEngine {
    _processor: AudioBufferProcessor,
    cancel_flag: Arc<AtomicBool>,
}

impl HellcallEngine {
    pub fn start(
        config: Config,
        model_path: &str,
        input_device_name: Option<String>,
        audio_dir: Option<String>,
    ) -> Result<Self> {
        // choose input device
        let input_device = if let Some(name) = input_device_name.filter(|n| !n.is_empty()) {
            name
        } else {
            let host = cpal::default_host();
            let default_device = host
                .default_input_device()
                .ok_or_else(|| anyhow!("No default input device found"))?;
            default_device.name()?
        };
        info!("input_device_name: {}", input_device);

        // choose audio dir
        let audio_dir = if let Some(dir) = audio_dir.filter(|d| !d.is_empty()) {
            dir
        } else {
            AUDIO_DIR.to_string()
        };

        // init
        let key_presser_config = config.key_presser.clone();
        let shortcut = config
            .commands
            .iter()
            .filter(|cmd| cmd.shortcut.is_some())
            .map(|cmd| (cmd.shortcut.clone().unwrap(), cmd.keys.clone()))
            .collect::<HashMap<_, _>>();
        let key_presser = Arc::new(KeyPresser::new(
            key_presser_config,
            config.key_map.clone(),
            shortcut,
        )?);
        let speaker = Arc::new(Speaker::new()?);

        let command_map: HashMap<String, Box<dyn Fn() + Send + Sync>> = config
            .commands
            .iter()
            .map(|cmd| -> Result<(String, Box<dyn Fn() + Send + Sync>)> {
                let key_presser_ref = Arc::clone(&key_presser);
                let speaker_ref = Arc::clone(&speaker);
                let keys = cmd.keys.clone();
                let audio_files = cmd.audio_files.clone();
                let audio_dir = audio_dir.clone();

                // check
                if cmd.command.is_empty() {
                    return Err(anyhow!("command must not be empty"));
                };
                KeyPresser::has_validity(keys.as_slice())?;

                Ok((
                    cmd.command.clone(),
                    Box::new(move || {
                        key_presser_ref.push(keys.as_slice());
                        if let Some(audio_path) = audio_files.choose(&mut rand::rng()) {
                            let audio_path = std::env::current_dir()
                                .unwrap()
                                .join(&audio_dir)
                                .join(audio_path);
                            let _ = speaker_ref.play_wav(audio_path.to_str().unwrap());
                        }
                    }) as Box<dyn Fn() + Send + Sync>,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        let command = Arc::new(Command::new(command_map));
        let command_dic = command.keys().map(|x| x.to_string()).collect::<Vec<_>>();

        let matcher = Arc::new(Mutex::new(LevenshteinMatcher::new(command_dic)));

        let trigger = config.trigger.clone();

        let mut audio_recognizer_config: AudioRecognizerConfig = config.recognizer.clone().into();
        let mut grammar: Vec<String> = config
            .commands
            .iter()
            .map(|cmd| {
                let grammar = cmd.grammar.clone();
                if let Some(grammar) = grammar {
                    if !grammar.is_empty() {
                        return grammar;
                    }
                }
                let command = cmd.command.clone();
                command.add_between_chars(" ")
            })
            .collect();

        if let Some(hit_word_grammar) = trigger.hit_word_grammar.clone() {
            grammar.push(hit_word_grammar);
        } else if !&trigger.hit_word.is_empty() {
            grammar.push(trigger.hit_word.clone().unwrap().add_between_chars(" "));
        }

        audio_recognizer_config.set_grammar(grammar);
        let recognizer = AudioRecognizer::new(model_path, audio_recognizer_config)?;
        let mut processor =
            AudioBufferProcessor::new_with_input_device_name(recognizer, input_device)?;

        let command_ref = Arc::clone(&command);
        let matcher_ref = Arc::clone(&matcher);

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag_clone = Arc::clone(&cancel_flag);

        let on_result = Box::new(move |result: RecognitionResult| {
            if cancel_flag_clone.load(Ordering::Relaxed) {
                return;
            }

            let speech = result.text.trim();
            if speech.is_empty() {
                return;
            }

            // process speech
            let speech = speech.replace(" ", "");
            let hit_word = trigger.hit_word.clone();
            let command_to_match = if hit_word.is_empty() {
                info!("speech: {}", speech);
                speech
            } else {
                let hit_word = hit_word.unwrap();
                if let Some(pos) = speech.rfind(hit_word.as_str()) {
                    let command_str = &speech[pos + hit_word.len()..];
                    info!("speech: {} {}", hit_word, command_str);
                    command_str.to_string()
                } else {
                    warn!("miss required word '{}': {}", hit_word, speech);
                    return;
                }
            };

            // match command
            if let Some(command) = matcher_ref
                .lock()
                .unwrap()
                .match_str(command_to_match.as_str())
            {
                info!("hit command: {}", command);
                command_ref.execute(command.as_str());
            } else {
                warn!("no matching command found: {}", command_to_match);
            }
        });

        processor.start(on_result)?;

        let key_presser_clone = Arc::clone(&key_presser);
        thread::spawn(move || {
            if let Err(e) = key_presser_clone.listen() {
                log::error!("Key presser error: {}", e);
            }
        });

        Ok(HellcallEngine {
            _processor: processor,
            cancel_flag,
        })
    }

    pub fn stop(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
    }
}
