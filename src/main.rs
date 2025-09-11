// #![allow(unused)]

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use inquire::Select;
use log::{info, warn};
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use vosk::LogLevel;

use config::*;
use core::audio::*;
use core::command::*;
use core::keypress::*;
use core::matcher::*;
use core::speaker::*;
use utils::*;

mod config;
pub mod core;
mod utils;

fn main() -> Result<()> {
    // print banner
    print_banner();

    // get env
    let model_path = env::var("VOSK_MODEL_PATH")?;
    let config_path = env::var("HELLCALL_CONFIG_PATH").unwrap_or("config.toml".to_string());
    let log_level = env::var("RUST_LOG")?;

    // init log
    env_logger::init();
    match log_level.as_str() {
        "info" => vosk::set_log_level(LogLevel::Info),
        "warn" => vosk::set_log_level(LogLevel::Warn),
        "error" => vosk::set_log_level(LogLevel::Error),
        _ => vosk::set_log_level(LogLevel::Info),
    }

    // load config
    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;

    // choose input device
    let input_device_name = get_input_device_name()?;
    info!("input_device_name: {}", input_device_name);

    // init
    let key_presser = Arc::new(KeyPresser::new(config.key_map));
    let speaker = Arc::new(Speaker::new()?);

    let command_map: HashMap<String, Box<dyn Fn() + Send + Sync>> = config
        .commands
        .iter()
        .map(|cmd| {
            let key_presser_ref = Arc::clone(&key_presser);
            let speaker_ref = Arc::clone(&speaker);
            let keys = cmd.keys.clone();
            let audio_files = cmd.audio_files.clone();
            (
                cmd.command.clone(),
                Box::new(move || {
                    key_presser_ref.push(keys.as_slice());
                    if let Some(audio_path) = audio_files.choose(&mut rand::rng()) {
                        let audio_path = std::env::current_dir()
                            .unwrap()
                            .join("audio")
                            .join(audio_path);
                        speaker_ref.play_wav(audio_path.to_str().unwrap()).unwrap();
                    }
                }) as Box<dyn Fn() + Send + Sync>,
            )
        })
        .collect();
    let command = Arc::new(Command::new(command_map));
    let command_dic = command.keys().map(|x| x.to_string()).collect::<Vec<_>>();

    let matcher = Arc::new(Mutex::new(LevenshteinMatcher::new(command_dic)));

    let trigger = config.trigger.clone();

    let mut audio_recognizer_config: AudioRecognizerConfig = config.recognizer.into();
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

    if let Some(hit_word_grammar) = trigger.hit_word_grammar {
        grammar.push(hit_word_grammar);
    } else if !&trigger.hit_word.is_empty() {
        grammar.push(trigger.hit_word.clone().unwrap().add_between_chars(" "));
    }

    audio_recognizer_config.set_grammar(grammar);
    let recognizer = AudioRecognizer::new(model_path.as_str(), audio_recognizer_config)?;
    let mut processor =
        AudioBufferProcessor::new_with_input_device_name(recognizer, input_device_name)?;

    let command_ref = Arc::clone(&command);
    let matcher_ref = Arc::clone(&matcher);
    let on_result = Box::new(move |result: RecognitionResult| {
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
    // block
    key_presser.listen()?;

    Ok(())
}

fn print_banner() {
    println!(
        r#"
_________________________________________
                                         
    /           /   /               /   /
---/__----__---/---/----__----__---/---/-
  /   ) /___) /   /   /   ' /   ) /   /  
_/___/_(___ _/___/___(___ _(___(_/___/___

HellCall v{} - Helldivers 2 语音指令工具
-----------------------------------------
 https://github.com/LyceumHewun/hellcall
 免费使用 · 禁止商用
 交流群: 1062683607
-----------------------------------------
    "#,
        env!("CARGO_PKG_VERSION")
    );
}

fn get_input_device_name() -> Result<String> {
    let host = cpal::default_host();

    if let Some(default_device) = host.default_input_device() {
        let device_name = default_device.name()?;
        if device_name.find("VB-Audio Virtual Cable").is_none() {
            return Ok(device_name.to_string());
        }
    }

    let devices = host.input_devices()?;
    let device_names = devices
        .into_iter()
        .map(|x| x.name().unwrap())
        .collect::<Vec<_>>();
    let device_name = Select::new("请选择麦克风设备", device_names).prompt()?;
    Ok(device_name)
}
