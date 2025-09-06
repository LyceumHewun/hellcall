#![allow(unused)]

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use vosk::{LogLevel, Model};

use core::audio::*;
use core::command::*;
use core::matcher::fuzzy::*;

mod core;

fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    vosk::set_log_level(LogLevel::Error);

    let model_path = env::args().nth(1).context("module path is empty")?;
    let grammar = vec![
        "呼叫",
        "飞 鹰",
        "五 百",
        "空袭",
        "集 束 弹",
        "汽油 弹",
        "烟雾弹",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut command_map: HashMap<&'static str, Box<dyn Fn() + Send + Sync>> = HashMap::new();
    // TODO:
    command_map.insert("呼叫飞鹰", Box::new(|| {
        println!("飞鹰已经就绪");
    }));
    command_map.insert("呼叫飞鹰五百", Box::new(|| {}));
    command_map.insert("呼叫飞鹰空袭", Box::new(|| {}));
    command_map.insert("呼叫飞鹰集束弹", Box::new(|| {}));
    command_map.insert("呼叫飞鹰汽油弹", Box::new(|| {}));
    command_map.insert("呼叫飞鹰烟雾弹", Box::new(|| {}));
    let command = Arc::new(Command::new(command_map));
    let command_dic = command.keys().map(|x| x.to_string()).collect::<Vec<_>>();

    let config = AudioRecognizerConfig {
        chunk_time: 0.2,
        grammar,
        vad_silence_duration: 500,
    };
    let recognizer = AudioRecognizer::new(model_path.as_str(), config)?;
    let mut processor = AudioBufferProcessor::new(recognizer)?;

    let matcher = Arc::new(FuzzyMatcher::new(command_dic));

    let command_ref = Arc::clone(&command);
    let matcher_ref = Arc::clone(&matcher);
    let on_result = Box::new(move |result: RecognitionResult| {
        let speech = result.text;
        if speech.is_empty() {
            return;
        }

        if let Some(command) = matcher_ref.match_str(&speech) {
            command_ref.execute(command);
        }
    });

    processor.start(on_result);

    // 阻塞线程
    std::thread::park();

    Ok(())
}
