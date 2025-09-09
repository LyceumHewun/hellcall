#![allow(unused)]

use std::collections::HashMap;
use rdev::Key;
use serde::Deserialize;

use crate::core::keypress::LocalKey;
use crate::core::audio::AudioRecognizerConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub recognizer: RecognizerConfig,
    /// 按键映射
    /// 
    /// 示例:
    /// ```toml
    /// [key_map]
    /// UP = "KeyW"
    /// DOWN = "KeyS"
    /// LEFT = "KeyA"
    /// RIGHT = "KeyD"
    /// CTRL = "ControlLeft"
    /// ```
    /// 
    /// 更多按键信息请参考: https://docs.rs/rdev/latest/rdev/enum.Key.html
    pub key_map: HashMap<LocalKey, Key>,
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
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommandConfig {
    pub command: String,
    pub grammar: Option<String>,
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
