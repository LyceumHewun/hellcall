use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rdev::{Event, EventType, Key, SimulateError, listen, simulate};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocalKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    CTRL,
}

pub struct KeyPresser {
    /// 按键映射
    key_map: HashMap<LocalKey, Key>,
    one_stack: Arc<Mutex<Option<Vec<LocalKey>>>>,
}

impl KeyPresser {
    pub fn new(key_map: HashMap<LocalKey, Key>) -> Self {
        Self {
            key_map,
            one_stack: Arc::new(Mutex::new(None)),
        }
    }

    pub fn push(&self, keys: &[LocalKey]) {
        *self.one_stack.lock().unwrap() = Some(keys.to_vec());
    }

    pub fn listen(&self) {
        let key_map = self.key_map.clone();
        let one_stack = Arc::clone(&self.one_stack);
        std::thread::spawn(move || {
            // 监听按键事件
            listen(move |event| {
                // 只处理按键事件
                if let EventType::KeyPress(key) = event.event_type {
                    // 处理 CTRL 键
                    if key == *key_map.get(&LocalKey::CTRL).unwrap() {
                        // 按顺序按下栈内按键
                        if let Some(keys) = one_stack.lock().unwrap().take() {
                            std::thread::sleep(std::time::Duration::from_millis(200));
                            for local_key in keys {
                                let key = *key_map.get(&local_key).unwrap();
                                simulate(&EventType::KeyPress(key)).unwrap();
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                simulate(&EventType::KeyRelease(key)).unwrap();
                                std::thread::sleep(std::time::Duration::from_millis(20));
                            }
                        }
                    }
                }
            })
        });
    }
}
