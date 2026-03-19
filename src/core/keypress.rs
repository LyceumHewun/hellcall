use anyhow::{Result, anyhow};
use log::info;
use rdev::{Button, EventType, Key, simulate};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::mpsc,
    sync::{Arc, Mutex},
    thread::JoinHandle,
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
    /// 扔出战备, 一般是鼠标左键
    THROW,
    /// 重新执行上一次键盘宏按键
    RESEND,
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
    tx: Option<mpsc::Sender<Vec<LocalKey>>>,
    worker_handle: Option<JoinHandle<()>>,
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
        let handle = std::thread::spawn({
            let config = Arc::clone(&config);
            let key_map = Arc::clone(&key_map);
            move || {
                while let Ok(mut keys) = rx.recv() {
                    info!("key pressed: {:?}", keys);

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
                                if let Err(e) = simulate(&event_type) {
                                    log::error!("simulate open-key press error: {:?}", e);
                                }
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
                            if let Err(e) = simulate(&event_press_type) {
                                log::error!("simulate key press error: {:?}", e);
                            }
                            std::thread::sleep(Duration::from_millis(config.key_release_interval));
                            if let Err(e) = simulate(&event_release_type) {
                                log::error!("simulate key release error: {:?}", e);
                            }
                            std::thread::sleep(Duration::from_millis(config.diff_key_interval));
                        }
                    }

                    // 模拟释放
                    if let Some(event_type) = open_key_event_release_type {
                        if let Err(e) = simulate(&event_type) {
                            log::error!("simulate open-key release error: {:?}", e);
                        }
                    }
                }
            }
        });

        Ok(Self {
            key_map,
            shortcut: Arc::new(shortcut),
            one_stack: Arc::new(Mutex::new(None)),
            spare_stack: Arc::new(Mutex::new(None)),
            tx: Some(tx),
            worker_handle: Some(handle),
        })
    }

    pub fn push(&self, keys: &[LocalKey]) {
        let keys = keys.to_vec();

        if let Some(first_key) = keys.first() {
            if first_key == &LocalKey::OPEN {
                if let Some(tx) = &self.tx {
                    if let Err(e) = tx.send(keys.clone()) {
                        log::error!("push send error: {:?}", e);
                    }
                }
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
        let tx = self.tx.as_ref().unwrap().clone();

        let open_key = self.key_map.get(&LocalKey::OPEN).unwrap().clone();
        let resend_key = self.key_map.get(&LocalKey::RESEND).unwrap().clone();
        // block
        rdev::listen(move |event| {
            // 只处理按下事件，其余直接返回，保持钩子回调轻量
            let Some(input) = (match event.event_type {
                EventType::KeyPress(key) => Some(Input::Key(key)),
                EventType::ButtonPress(key) => Some(Input::Button(key)),
                _ => None,
            }) else {
                return;
            };

            if input == open_key {
                // 使用 try_lock 非阻塞：若锁被占用则跳过，绝不阻塞系统钩子
                if let Ok(mut guard) = one_stack.try_lock() {
                    if let Some(keys) = guard.take() {
                        if let Err(e) = tx.send(keys) {
                            log::error!("listen send error: {:?}", e);
                        }
                    }
                }
            } else if input == resend_key {
                // 先读 spare_stack，再写 one_stack，避免同时持有两把锁（防死锁）
                let keys_opt = spare_stack.try_lock().ok().and_then(|g| g.clone());
                if let Some(keys) = keys_opt {
                    info!("resend key press: {:?}", &keys);
                    if let Ok(mut guard) = one_stack.try_lock() {
                        guard.replace(keys);
                    }
                }
            } else if let Some(keys) = shortcut.get(&input) {
                // 检查是否是快捷键
                if let Err(e) = tx.send(keys.clone()) {
                    log::error!("shortcut send error: {:?}", e);
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
            return Err(anyhow!("cannot use RESEND key in a macro"));
        }

        let has_open = keys.contains(&LocalKey::OPEN);
        let has_throw = keys.contains(&LocalKey::THROW);

        // OPEN 必须在第一位
        if has_open && keys.first() != Some(&LocalKey::OPEN) {
            return Err(anyhow!("OPEN key must be the first key"));
        }

        // THROW 必须在最后一位
        if has_throw && keys.last() != Some(&LocalKey::THROW) {
            return Err(anyhow!("THROW key must be the last key"));
        }

        // 中间段不允许再出现 OPEN 或 THROW
        if keys.len() > 2 {
            for key in &keys[1..keys.len() - 1] {
                if key == &LocalKey::OPEN {
                    return Err(anyhow!("OPEN key must be the first key"));
                }
                if key == &LocalKey::THROW {
                    return Err(anyhow!("THROW key must be the last key"));
                }
            }
        }

        Ok(())
    }
}

impl Drop for KeyPresser {
    fn drop(&mut self) {
        // Drop tx first to close the channel: the worker thread's rx.recv()
        // will return Err and the while-loop exits naturally.
        drop(self.tx.take());
        // Then join to wait for the worker thread to fully exit.
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}
