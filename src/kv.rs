use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// `Result` will contain the result of a `KvStore` operation
pub type Result<T> = std::result::Result<T, Error>;

/// `KvStore` is a simple-to-use, efficient key value store
pub struct KvStore {
    path: PathBuf,
    store: fs::File,
    map: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Command {
    Write { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    /// Opens a database file
    pub fn open(path: &Path) -> Result<KvStore> {
        let path = path.join("db.json");
        let file = fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&path)?;

        let mut kv_store = KvStore {
            path,
            store: file,
            map: HashMap::new(),
        };

        kv_store.create_map()?;

        Ok(kv_store)
    }

    /// sets or updates the value in the key value store at the requested key
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// use std::path::Path;
    /// let mut kv = KvStore::open(Path::new(".")).expect("Could not open file");
    /// kv.set("key".to_owned(), "value".to_owned()).expect("Could not set key");
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.write_record(Command::Write {
            key: key.to_owned(),
            value: value.to_owned(),
        })?;
        Ok(())
    }

    /// `get(key)` retrieves the value in the key value store at the requested key
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// use std::path::Path;
    /// let mut kv = KvStore::open(Path::new(".")).expect("Could not open file");;
    /// kv.set("key".to_owned(), "value".to_owned()).expect("Could not set value");;
    /// assert_eq!(Some("value".to_owned()), kv.get("key".to_owned()).expect("Could not get key"));
    /// ```
    //     - "get"
    //   - The user invokes `kvs get mykey`
    //   - `kvs` reads the entire log, one command at a time, recording the
    //    affected key and file offset of the command to an in-memory _key -> log
    //     pointer_ map
    //   - It then checks the map for the log pointer
    //   - If it fails, it prints "Key not found", and exits with exit code 0
    //   - If it succeeds
    //     - It deserializes the command to get the last recorded value of the key
    //     - It prints the value to stdout and exits with exit code 0
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let position = self.map.get(&key).cloned();
        match position {
            Some(position) => {
                let record = self.read_record(position)?;
                match record {
                    Some(Command::Write { key: _, value }) => Ok(Some(value.to_owned())),
                    _ => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    /// `get(key)` removes the value in the key value store at the requested key
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// use std::path::Path;
    /// let mut kv = KvStore::open(Path::new(".")).expect("Could not open file");;
    /// kv.set("key".to_owned(), "value".to_owned());
    /// kv.remove("key".to_owned()).expect("Could not remove key");;
    /// assert_eq!(None, kv.get("key".to_owned()).expect("could not get key"));
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        let value = self.get(key.to_owned())?;
        match value {
            Some(_) => {
                self.write_record(Command::Remove {
                    key: key.to_owned(),
                })?;
                Ok(())
            }
            None => Err(failure::err_msg("Key not found")),
        }
    }

    fn write_record(&mut self, command: Command) -> Result<()> {
        let mut file = &self.store;
        let position = file.seek(SeekFrom::End(0))?;
        {
            let mut command = serde_json::to_string(&command)?;
            command.push_str("\n");

            file.write_all(command.as_bytes())?;
        }
        match command {
            Command::Write { key, value: _ } => {
                self.map.insert(key.to_owned(), position);
            }
            Command::Remove { key } => {
                self.map.remove(&key);
            }
        }
        
        self.compact_log()?;
        self.create_map()?;

        Ok(())
    }

    fn read_record(&mut self, position: u64) -> Result<Option<Command>> {
        let mut reader = BufReader::new(&self.store);
        reader.seek(SeekFrom::Start(position))?;
        let mut buffer = String::new();
        let len = reader.read_line(&mut buffer)?;
        if len == 0 {
            return Ok(None);
        }
        Ok(Some(serde_json::from_str(&buffer)?))
    }

    fn create_map(&mut self) -> Result<()> {
        let mut reader = BufReader::new(&self.store);
        reader.seek(SeekFrom::Start(0))?;
        loop {
            let mut buffer = String::new();
            let position = reader.seek(SeekFrom::Current(0))?;
            let result = reader.read_line(&mut buffer)?;
            match result {
                0 => break,
                _ => {
                    let command = serde_json::from_str(&buffer);
                    match command {
                        Ok(Command::Write { key, value: _ }) => {
                            self.map.insert(key.to_owned(), position);
                        }
                        Ok(Command::Remove { key }) => {
                            self.map.remove(&key);
                        }
                        Err(_) => continue,
                    }
                }
            }
        }

        Ok(())
    }

    fn compact_log(&mut self) -> Result<()> {
        let current_file_name = &self.path;
        let mut new_file_name = Path::new(&self.path).to_path_buf();
        new_file_name.set_extension("json.temp");
        let mut reader = BufReader::new(&self.store);
        reader.seek(SeekFrom::Start(0))?;
        let mut new_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(Path::new(&new_file_name))?;
        for (_, position) in &self.map {
            let mut buffer = String::new();
            reader.seek(SeekFrom::Start(*position))?;
            reader.read_line(&mut buffer)?;
            buffer.push_str("\n");
            new_file.write(&buffer.as_bytes())?;
        }

        fs::rename(new_file_name, current_file_name)?;
        self.store = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&current_file_name)?;
        Ok(())
    }
}
