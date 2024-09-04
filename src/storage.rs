use serde_json::{json, Value};
use std::{
    fs::File,
    io::{Read, Write},
};

pub trait Storage {
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), String>;
}
pub struct JsonStorage {
    filepath: String,
}

impl JsonStorage {
    pub fn new(filepath: &str) -> Result<JsonStorage, String> {
        if let Ok(mut file) = File::create_new(&filepath) {
            file.write_all("{}".as_bytes()).map_err(|err| {
                format!("failed to instantiate storage file {}: {}", filepath, err)
            })?;
        }
        Ok(JsonStorage {
            filepath: filepath.to_owned(),
        })
    }
}

impl Storage for JsonStorage {
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), String> {
        dbg!(branch_prefix); // TODO

        let mut file = File::open(&self.filepath)
            .map_err(|err| format!("cannot open storage file {}: {}", self.filepath, err))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|err| format!("cannot read storage file {}: {}", self.filepath, err))?;
        let mut json_data: Value = serde_json::from_str(&content)
            .map_err(|err| format!("storage file is not valid JSON {}: {}", self.filepath, err))?;
        // todo here
        Ok(())
    }
}
