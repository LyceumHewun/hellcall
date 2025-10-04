use anyhow::{Result, anyhow};
use log::info;
use rdev::{Button, EventType, Key, simulate};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::mpsc,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Button(Button), // 让 Unknown 优先绑定到Button
    Key(Key),
}

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
    /// 扔出战备, 一般是鼠标左键
    THROW,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPresserConfig {
    /// 等待打开战备页面的时间
    pub wait_open_time: u64,
    /// 按键释放间隔
    pub key_release_interval: u64,
    /// 按键间隔
    pub diff_key_interval: u64,
}

impl Default for KeyPresserConfig {
    fn default() -> Self {
        Self {
            wait_open_time: 400,
            key_release_interval: 50,
            diff_key_interval: 20,
        }
    }
}

pub struct KeyPresser {
    /// 按键映射
    key_map: Arc<HashMap<LocalKey, Input>>,
    shortcut: Arc<HashMap<Input, Vec<LocalKey>>>,
    one_stack: Arc<Mutex<Option<Vec<LocalKey>>>>,
    spare_stack: Arc<Mutex<Option<Vec<LocalKey>>>>,
    tx: mpsc::Sender<Vec<LocalKey>>,
}

impl KeyPresser {
    pub fn new(
        config: KeyPresserConfig,
        key_map: HashMap<LocalKey, Input>,
        shortcut: HashMap<Input, Vec<LocalKey>>,
    ) -> Result<Self> {
        // 检查所有 LocalKey 是否都在 key_map 中
        for local_key in [
            LocalKey::UP,
            LocalKey::DOWN,
            LocalKey::LEFT,
            LocalKey::RIGHT,
            LocalKey::OPEN,
            LocalKey::RESEND,
            LocalKey::THROW,
        ] {
            if !key_map.contains_key(&local_key) {
                return Err(anyhow!("Missing mapping for LocalKey: {:?}", local_key));
            }
        }

        // keypress worker
        let (tx, rx) = std::sync::mpsc::channel::<Vec<LocalKey>>();
        let config = Arc::new(config);
        let key_map = Arc::new(key_map);
        std::thread::spawn({
            let config = Arc::clone(&config);
            let key_map = Arc::clone(&key_map);
            move || {
                while let Ok(keys) = rx.recv() {
                    info!("key pressed: {:?}", keys);
                    let mut keys = keys.clone();

                    // check first key is open key
                    let mut open_key_event_release_type: Option<EventType> = None;
                    if let Some(first_key) = keys.first() {
                        if first_key == &LocalKey::OPEN {
                            if let Some(key) = key_map.get(first_key) {
                                let event_type = match key {
                                    Input::Key(key) => {
                                        open_key_event_release_type =
                                            Some(EventType::KeyRelease(*key));
                                        EventType::KeyPress(*key)
                                    }
                                    Input::Button(button) => {
                                        open_key_event_release_type =
                                            Some(EventType::ButtonRelease(*button));
                                        EventType::ButtonPress(*button)
                                    }
                                };

                                // 模拟按下
                                simulate(&event_type).unwrap();
                            }

                            // 移除第一个按键
                            keys.remove(0);
                        }
                    }

                    std::thread::sleep(Duration::from_millis(config.wait_open_time));
                    for local_key in keys {
                        if let Some(key) = key_map.get(&local_key) {
                            let (event_press_type, event_release_type) = match key {
                                Input::Key(key) => {
                                    (EventType::KeyPress(*key), EventType::KeyRelease(*key))
                                }
                                Input::Button(button) => (
                                    EventType::ButtonPress(*button),
                                    EventType::ButtonRelease(*button),
                                ),
                            };

                            // 模拟按下和释放
                            simulate(&event_press_type).unwrap();
                            std::thread::sleep(Duration::from_millis(config.key_release_interval));
                            simulate(&event_release_type).unwrap();
                            std::thread::sleep(Duration::from_millis(config.diff_key_interval));
                        }
                    }

                    // 模拟释放
                    if let Some(event_type) = open_key_event_release_type {
                        simulate(&event_type).unwrap();
                    }
                }
            }
        });

        Ok(Self {
            key_map,
            shortcut: Arc::new(shortcut),
            one_stack: Arc::new(Mutex::new(None)),
            spare_stack: Arc::new(Mutex::new(None)),
            tx,
        })
    }

    pub fn push(&self, keys: &[LocalKey]) {
        let keys = keys.to_vec();

        if let Some(first_key) = keys.first() {
            if first_key == &LocalKey::OPEN {
                self.tx.send(keys.clone()).unwrap();
            } else {
                *self.one_stack.lock().unwrap() = Some(keys.clone());
                *self.spare_stack.lock().unwrap() = Some(keys.clone());
            }
        }
    }

    /// block
    pub fn listen(&self) -> Result<()> {
        let shortcut = Arc::clone(&self.shortcut);
        let one_stack = Arc::clone(&self.one_stack);
        let spare_stack = Arc::clone(&self.spare_stack);
        let tx = self.tx.clone();

        let open_key = self.key_map.get(&LocalKey::OPEN).unwrap().clone();
        let resend_key = self.key_map.get(&LocalKey::RESEND).unwrap().clone();
        // block
        rdev::listen(move |event| {
            if let Some(input) = match event.event_type {
                EventType::KeyPress(key) => Some(Input::Key(key)),
                EventType::ButtonPress(key) => Some(Input::Button(key)),
                _ => None,
            } {
                if input == open_key {
                    if let Some(keys) = one_stack.lock().unwrap().take() {
                        tx.send(keys).unwrap();
                    }
                } else if input == resend_key {
                    if let Some(keys) = spare_stack.lock().unwrap().clone() {
                        info!("resend key press: {:?}", &keys);
                        one_stack.lock().unwrap().replace(keys);
                    }
                } else {
                    // 检查是否是快捷键
                    if let Some(keys) = shortcut.get(&input) {
                        tx.send(keys.clone()).unwrap();
                    }
                }
            }
        })
        .map_err(|err| anyhow!("listen key press error: {:?}", err))?;

        Ok(())
    }

    pub fn has_validity(keys: &[LocalKey]) -> Result<()> {
        if keys.is_empty() {
            return Err(anyhow!("keys must not be empty"));
        }

        if keys.contains(&LocalKey::RESEND) {
            return Err(anyhow!("can not use resend key"));
        }

        if keys.contains(&LocalKey::OPEN) || keys.contains(&LocalKey::THROW) {
            if keys.last() == Some(&LocalKey::OPEN) {
                return Err(anyhow!("open key must be first"));
            }

            if keys.first() == Some(&LocalKey::THROW) {
                return Err(anyhow!("throw key must be last"));
            }

            if keys.len() > 2 {
                let mid = &keys[1..keys.len() - 1];
                for key in mid {
                    if key == &LocalKey::THROW {
                        return Err(anyhow!("throw key must be last"));
                    }
                    if key == &LocalKey::OPEN {
                        return Err(anyhow!("open key must be first"));
                    }
                }
            }
        }

        Ok(())
    }
}
