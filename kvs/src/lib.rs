extern crate failure;
extern crate serde;

use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Drop;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
    file: File,
}

impl KvStore {
    pub fn new() -> KvStore {
        let cwd = env::current_dir().unwrap();
        KvStore::open(cwd.as_path()).unwrap()
    }

    pub fn set(self: &mut KvStore, key: String, value: String) -> Result<()> {
        let set_command = SetCommand { key, value };
        self.file.write_all(set_command.to_json().as_ref());
        self.map.insert(set_command.key, set_command.value);
        Ok(())
    }

    pub fn get(self: &KvStore, key: String) -> Result<Option<String>> {
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    pub fn remove(self: &mut KvStore, key: String) -> Result<()> {
        let rm_command = RmCommand { key };
        self.file.write_all(rm_command.to_json().as_ref());
        self.map.remove(&rm_command.key);
        Ok(())
    }

    pub fn open(dir: &Path) -> Result<KvStore> {
        let filename = dir.join(Path::new("data.db"));
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;
        Ok(KvStore {
            map: HashMap::new(),
            file: file,
        })
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        drop(&self.file);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    pub fn new(key: String, value: String) -> SetCommand {
        SetCommand { key, value }
    }

    pub fn to_json(self: &SetCommand) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RmCommand {
    key: String,
}

impl RmCommand {
    pub fn new(key: String) -> RmCommand {
        RmCommand { key }
    }

    pub fn to_json(self: &RmCommand) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommand {
    key: String,
}
