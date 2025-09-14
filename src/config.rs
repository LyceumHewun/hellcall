#![allow(unused)]

use serde::Deserialize;
use std::collections::HashMap;

use crate::core::audio::AudioRecognizerConfig;
use crate::core::keypress::{Input, KeyPresserConfig, LocalKey};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub recognizer: RecognizerConfig,
    pub key_presser: KeyPresserConfig,
    /// 按键映射
    ///
    /// 示例:
    /// ```toml
    /// [key_map]
    /// UP = "KeyW"
    /// DOWN = "KeyS"
    /// LEFT = "KeyA"
    /// RIGHT = "KeyD"
    /// OPEN = "ControlLeft"
    /// ```
    ///
    /// 更多按键信息请参考: https://docs.rs/rdev/latest/rdev/enum.Key.html
    pub key_map: HashMap<LocalKey, Input>,
    pub trigger: TriggerConfig,
    pub commands: Vec<CommandConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecognizerConfig {
    /// 音频识别的时间段 (秒)
    pub chunk_time: f32,
    /// 判断语音结束后的静音持续时间 (毫秒)
    pub vad_silence_duration: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TriggerConfig {
    pub hit_word: Option<String>,
    pub hit_word_grammar: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommandConfig {
    pub command: String,
    pub grammar: Option<String>,
    pub shortcut: Option<Input>,
    pub keys: Vec<LocalKey>,
    pub audio_files: Vec<String>,
}

impl Into<AudioRecognizerConfig> for RecognizerConfig {
    fn into(self) -> AudioRecognizerConfig {
        AudioRecognizerConfig {
            chunk_time: self.chunk_time,
            grammar: Vec::new(),
            vad_silence_duration: self.vad_silence_duration,
        }
    }
}
