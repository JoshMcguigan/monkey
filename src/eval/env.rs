use std::collections::HashMap;
use crate::eval::Object;

pub struct Env {
    env: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            env: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.env.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        self.env.get(key).map(|val| val.clone())
    }
}
