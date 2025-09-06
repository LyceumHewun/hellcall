#![allow(unused)]

use anyhow::Result;
use vosk::{LogLevel, Model};

use core::audio::*;

mod core;

fn main() -> Result<()> {
    vosk::set_log_level(LogLevel::Error);

    let model_path = "C:\\Users\\qlan\\Downloads\\vosk-model-small-cn-0.22";
    let grammar = vec![
        "呼叫",
        "飞 鹰",
        "轨道",
        "火",
        "加特林",
        "支援",
        "补给",
        "[unk]",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let config = AudioRecognizerConfig {
        chunk_time: 0.2,
        grammar,
        vad_silence_duration: 500,
    };

    let recognizer = AudioRecognizer::new(model_path, config)?;

    let mut processor = AudioBufferProcessor::new(recognizer)?;

    processor.start(Box::new(|result| {
        if result.text.is_empty() {
            return;
        }
        println!("{:?}", result.text);
    }));

    // 阻塞线程
    std::thread::park();

    Ok(())
}
