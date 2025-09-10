use anyhow::{Result, anyhow};
use log::info;
use rdev::{EventType, Key, simulate};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    /// 打开战备页面按键
    OPEN,
    /// 重新执行上一次键盘宏按键
    RESEND,
}

pub struct KeyPresser {
    /// 按键映射
    key_map: Arc<HashMap<LocalKey, Key>>,
    one_stack: Arc<Mutex<Option<Vec<LocalKey>>>>,
    spare_stack: Arc<Mutex<Option<Vec<LocalKey>>>>,
}

impl KeyPresser {
    pub fn new(key_map: HashMap<LocalKey, Key>) -> Self {
        Self {
            key_map: Arc::new(key_map),
            one_stack: Arc::new(Mutex::new(None)),
            spare_stack: Arc::new(Mutex::new(None)),
        }
    }

    pub fn push(&self, keys: &[LocalKey]) {
        *self.one_stack.lock().unwrap() = Some(keys.to_vec());
        *self.spare_stack.lock().unwrap() = Some(keys.to_vec());
    }

    /// block
    pub fn listen(&self) -> Result<()> {
        let key_map = Arc::clone(&self.key_map);
        let one_stack = Arc::clone(&self.one_stack);
        let spare_stack = Arc::clone(&self.spare_stack);

        // keypress worker
        let (tx, rx) = std::sync::mpsc::channel::<Vec<LocalKey>>();
        std::thread::spawn({
            let key_map = Arc::clone(&key_map);
            move || {
                while let Ok(keys) = rx.recv() {
                    info!("key pressed: {:?}", keys);
                    std::thread::sleep(Duration::from_millis(400));
                    for local_key in keys {
                        if let Some(&key) = key_map.get(&local_key) {
                            simulate(&EventType::KeyPress(key)).unwrap();
                            std::thread::sleep(Duration::from_millis(50));
                            simulate(&EventType::KeyRelease(key)).unwrap();
                            std::thread::sleep(Duration::from_millis(20));
                        }
                    }
                }
            }
        });

        // block
        rdev::listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                if key == *key_map.get(&LocalKey::OPEN).unwrap() {
                    if let Some(keys) = one_stack.lock().unwrap().take() {
                        tx.send(keys).unwrap();
                    }
                } else if key == *key_map.get(&LocalKey::RESEND).unwrap() {
                    if let Some(keys) = spare_stack.lock().unwrap().clone() {
                        info!("resend key press: {:?}", &keys);
                        one_stack.lock().unwrap().replace(keys);
                    }
                }
            }
        })
        .map_err(|err| anyhow!("listen key press error: {:?}", err))?;

        Ok(())
    }
}
