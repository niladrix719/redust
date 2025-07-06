use std::collections::HashMap;

pub struct MapStore {
    map: HashMap<String, String>
}

impl MapStore {
    pub fn new() -> Self {
        MapStore {
            map: HashMap::new(), 
        }
    }
    pub fn set_val(&mut self, key: &str, val: &str) {
        self.map.insert(key.to_string(), val.to_string());
    }
    pub fn get_val(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
} 
