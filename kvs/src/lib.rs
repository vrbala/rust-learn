extern crate failure;

use failure::Error;
use std::collections::HashMap;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

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

    pub fn set(self: &mut KvStore, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn get(self: &KvStore, key: String) -> Result<Option<String>> {
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    pub fn remove(self: &mut KvStore, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    pub fn open(dir: &Path) -> Result<KvStore> {
        unimplemented!();
    }
}
