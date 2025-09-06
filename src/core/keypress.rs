use log::{info, warn};
use rdev::{Event, EventType, Key, SimulateError, listen, simulate};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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

        let ctrl_flag = Arc::new(Mutex::new(false)); // 标记 Ctrl 是否按下
        let ctrl_flag_clone = ctrl_flag.clone();
        std::thread::spawn(move || {
            // 监听按键事件
            listen(move |event| {
                // 只处理按键事件
                if let EventType::KeyPress(key) = event.event_type {
                    // 处理 CTRL 键
                    if key == *key_map.get(&LocalKey::CTRL).unwrap() {
                        let mut flag = ctrl_flag_clone.lock().unwrap();
                        if !*flag {
                            *flag = true;

                            // 按顺序按下栈内按键
                            let keys = one_stack.lock().unwrap().take();
                            if let Some(keys) = keys {
                                let key_map_clone = key_map.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_millis(400));
                                    info!("press {:?}", &keys);
                                    for local_key in keys {
                                        let key = *key_map_clone.get(&local_key).unwrap();
                                        simulate(&EventType::KeyPress(key)).unwrap();
                                        std::thread::sleep(std::time::Duration::from_millis(50));
                                        simulate(&EventType::KeyRelease(key)).unwrap();
                                        std::thread::sleep(std::time::Duration::from_millis(20));
                                    }
                                });
                            }
                        }
                    }
                }
                if let EventType::KeyRelease(key) = event.event_type {
                    if key == *key_map.get(&LocalKey::CTRL).unwrap() {
                        let mut flag = ctrl_flag_clone.lock().unwrap();
                        if *flag {
                            *flag = false;
                        }
                    }
                }
            })
        });
    }
}
