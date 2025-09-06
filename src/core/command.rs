use std::{collections::HashMap, ops::Deref};

pub struct Command {
    map: HashMap<&'static str, Box<dyn Fn() + Send + Sync>>,
}

impl Deref for Command {
    type Target = HashMap<&'static str, Box<dyn Fn() + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Command {
    pub fn new(map: HashMap<&'static str, Box<dyn Fn() + Send + Sync>>) -> Self {
        Self { map }
    }

    pub fn execute(&self, command: &str) {
        if let Some(f) = self.map.get(command) {
            f();
        }
    }
}
