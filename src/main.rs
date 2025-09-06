#![allow(unused)]

use anyhow::{Context, Result};
use rdev::Key;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use vosk::{LogLevel, Model};
use log::{info, warn};

use core::audio::*;
use core::command::*;
use core::keypress::*;
use core::keypress::LocalKey::{CTRL, DOWN, LEFT, RIGHT, UP};
use core::matcher::fuzzy::*;
use core::speaker::*;

mod core;

fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    vosk::set_log_level(LogLevel::Error);
    env_logger::init();

    let model_path = env::args().nth(1).context("module path is empty")?;
    let grammar = vec![
        "呼叫",
        "飞 鹰",
        "五 百",
        "空袭",
        "集 束 弹",
        "汽油 弹",
        "烟雾弹",
        "一百 一",
        "补给",
        "补给 包",
        "增援",
        "轨道",
        "火",
        "加特林",
        "榴弹 枪",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut key_map: HashMap<LocalKey, Key> = HashMap::new();
    key_map.insert(UP, Key::KeyW);
    key_map.insert(DOWN, Key::KeyS);
    key_map.insert(LEFT, Key::KeyA);
    key_map.insert(RIGHT, Key::KeyD);
    key_map.insert(CTRL, Key::ControlLeft);
    let key_presser = Arc::new(KeyPresser::new(key_map));
    let speaker = Arc::new(Speaker::new()?);

    let command_map= [
        /// 飞鹰
        ("呼叫飞鹰", vec![UP, RIGHT, RIGHT], "eagle.wav"),
        ("呼叫飞鹰五百", vec![UP, RIGHT, DOWN, DOWN, DOWN], "eagle.wav"),
        ("呼叫飞鹰空袭", vec![UP, RIGHT, DOWN, LEFT], "eagle.wav"),
        ("呼叫飞鹰集束弹", vec![UP, RIGHT, DOWN, DOWN, RIGHT], "eagle.wav"),
        ("呼叫飞鹰汽油弹", vec![UP, RIGHT, DOWN, UP], "eagle.wav"),
        ("呼叫飞鹰烟雾弹", vec![UP, RIGHT, UP, DOWN], "eagle.wav"),
        ("呼叫飞鹰一百一", vec![UP, RIGHT, UP, LEFT], "eagle.wav"),
        /// 补给
        ("呼叫补给", vec![DOWN, DOWN, UP, RIGHT], "resupply.wav"),
        ("呼叫补给包", vec![DOWN, LEFT, DOWN, UP, UP, DOWN], "resupply.wav"),
        /// 轨道武器
        ("呼叫轨道火", vec![RIGHT, RIGHT, DOWN, LEFT, RIGHT, UP], "orbital_napalm_barrage.wav"),
        /// 3武
        ("呼叫榴弹枪", vec![DOWN, LEFT, UP, LEFT, DOWN], "resupply.wav"),
        /// mission
        ("呼叫增援", vec![UP, DOWN, RIGHT, LEFT, UP], "reinforce.wav"),
    ];
    let mut command_hashmap: HashMap<&'static str, Box<dyn Fn() + Send + Sync>> = HashMap::new();
    for (command, keys, audio_path) in command_map {
        let key_presser_ref = Arc::clone(&key_presser);
        let speaker_ref = Arc::clone(&speaker);
        command_hashmap.insert(command, Box::new(move || {
            info!("push key presses: {:?}", keys);
            &key_presser_ref.push(keys.as_slice());

            let audio_path = std::env::current_dir().unwrap().join("audio").join(audio_path);
            info!("play audio: {:?}", audio_path);
            speaker_ref.play_wav(audio_path.to_str().unwrap()).unwrap();
        }));
    }

    let command = Arc::new(Command::new(command_hashmap));
    let command_dic = command.keys().map(|x| x.to_string()).collect::<Vec<_>>();

    let config = AudioRecognizerConfig {
        chunk_time: 0.2, // 0.2 秒识别一次
        grammar,
        vad_silence_duration: 500,
    };
    let recognizer = AudioRecognizer::new(model_path.as_str(), config)?;
    let mut processor = AudioBufferProcessor::new(recognizer)?;

    let matcher = Arc::new(FuzzyMatcher::new(command_dic));

    let command_ref = Arc::clone(&command);
    let matcher_ref = Arc::clone(&matcher);
    let on_result = Box::new(move |result: RecognitionResult| {
        let speech = result.text.trim();
        if speech.is_empty() {
            return;
        }

        info!("speech: {}", speech);

        // 首句触发
        if !speech.starts_with("呼叫") {
            warn!("miss required word '呼叫': {}", speech);
            return;
        }

        if let Some(command) = matcher_ref.match_str(&speech) {
            info!("hit command: {}", command);
            command_ref.execute(command);
        }
    });

    processor.start(on_result);
    key_presser.listen(); // 监听键盘事件

    info!("startup success");

    // 阻塞线程
    std::thread::park();

    Ok(())
}
