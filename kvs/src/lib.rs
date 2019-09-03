
use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(self : &mut KvStore,
               key : String, value : String) {
        self.map.insert(key, value);
    }

    pub fn get(self : &KvStore,
               key : String) -> Option<String> {
        match self.map.get(&key)  {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn remove(self : &mut KvStore,
                  key : String) {
        self.map.remove(&key);
    }
}
