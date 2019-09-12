extern crate failure;
extern crate serde;

use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
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
        let set_command = Command::SetCommand { key, value };
        self.file
            .write_all(serde_json::to_string(&set_command).unwrap().as_ref())?;
        self.file.write_all(b"\n")?;
        if let Command::SetCommand { key, value } = set_command {
            self.map.insert(key, value);
        }
        Ok(())
    }

    pub fn get(self: &KvStore, key: String) -> Result<Option<String>> {
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    pub fn remove(self: &mut KvStore, key: String) -> Result<()> {
        let rm_command = Command::RmCommand { key };
        self.file
            .write_all(serde_json::to_string(&rm_command).unwrap().as_ref())?;
        if let Command::RmCommand { key } = rm_command {
            self.map.remove(&key);
        }
        Ok(())
    }

    pub fn open(dir: &Path) -> Result<KvStore> {
        let filename = dir.join(Path::new("data.db"));
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;

        let mut map = HashMap::new();
        let buffer = BufReader::new(&file);
        for line in buffer.lines() {
            let cmd = serde_json::from_str(&line.unwrap())?;
            match cmd {
                Command::SetCommand { key, value } => map.insert(key, value),
                Command::RmCommand { key } => map.remove(&key),
                _ => panic!("Corrupt data in the db file."),
            };
        }
        Ok(KvStore { map, file })
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        drop(&self.file);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    SetCommand { key: String, value: String },
    RmCommand { key: String },
    GetCommand { key: String },
}
