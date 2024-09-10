use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Stored {
    branches: Option<Vec<String>>,
}

pub trait Storage {
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), StorageError>;
    fn list_prefixes(&self) -> Result<Vec<String>, StorageError>;
}

pub struct StorageError {
    message: String,
}

impl StorageError {
    pub fn new(message: &str) -> StorageError {
        StorageError {
            message: message.to_owned(),
        }
    }
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
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), StorageError> {
        let mut file = File::open(&self.filepath)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut json_data: Stored = serde_json::from_str(&content)?;
        let mut branches = json_data.branches.take().unwrap_or(vec![]);
        if branches.iter().find(|s| *s == branch_prefix).is_some() {
            return Ok(());
        }

        branches.push(branch_prefix.to_owned());
        json_data.branches = Some(branches);

        let file = File::create(&self.filepath)?;
        serde_json::to_writer_pretty(file, &json_data)?;
        Ok(())
    }

    fn list_prefixes(&self) -> Result<Vec<String>, StorageError> {
        let mut content = String::new();
        File::open(&self.filepath)?.read_to_string(&mut content)?;
        let json_data: Stored = serde_json::from_str(&content)?;
        if let Some(branches) = json_data.branches {
            Ok(branches)
        } else {
            Ok(vec![])
        }
    }
}

impl From<String> for StorageError {
    fn from(value: String) -> Self {
        StorageError::new(&value)
    }
}

impl From<std::io::Error> for StorageError {
    fn from(value: std::io::Error) -> Self {
        StorageError::new(&format!("IO error: {}", value))
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        StorageError::new(&format!("JSON error: {}", value))
    }
}

impl From<StorageError> for String {
    fn from(value: StorageError) -> Self {
        value.message
    }
}
