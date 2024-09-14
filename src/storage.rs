use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Stored {
    branches: Option<Vec<BranchInfo>>,
}

pub trait Storage {
    fn push_branch_info(&self, info: &BranchInfo) -> Result<(), StorageError>;
    fn list_branch_info(&self) -> Result<Vec<BranchInfo>, StorageError>;
    fn trim_to_latest(&self, n: usize) -> Result<(), StorageError>;
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BranchInfo {
    pub prefix: String,
    pub name: String,
    #[serde(with = "ts_seconds")]
    pub last_used: DateTime<Utc>,
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

    fn read_from_json(&self) -> Result<Stored, StorageError> {
        let mut file = File::open(&self.filepath)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }

    fn write_to_json(&self, data: &Stored) -> Result<(), StorageError> {
        let file = File::create(&self.filepath)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }
}

impl Storage for JsonStorage {
    fn push_branch_info(&self, info: &BranchInfo) -> Result<(), StorageError> {
        let mut data = self.read_from_json()?;
        let branches = data.branches.take().unwrap_or(vec![]);

        // If there is a branch with the same prefix, then replace the existing branch with the new branch.
        let mut found_existing_with_same_prefix = false;
        let mut branches: Vec<BranchInfo> = branches
            .into_iter()
            .map(|b| {
                if b.prefix == info.prefix {
                    found_existing_with_same_prefix = true;
                    info.clone()
                } else {
                    b
                }
            })
            .collect();
        if !found_existing_with_same_prefix {
            branches.push(info.clone());
        }
        data.branches = Some(branches);
        self.write_to_json(&data)?;
        Ok(())
    }

    fn list_branch_info(&self) -> Result<Vec<BranchInfo>, StorageError> {
        let mut content = String::new();
        File::open(&self.filepath)?.read_to_string(&mut content)?;
        let data: Stored = serde_json::from_str(&content)?;
        if let Some(info_list) = data.branches {
            Ok(info_list)
        } else {
            Ok(vec![])
        }
    }

    fn trim_to_latest(&self, n: usize) -> Result<(), StorageError> {
        let mut data = self.read_from_json()?;
        if data.branches.is_none() {
            return Ok(());
        }
        let mut branches = data.branches.unwrap();
        branches.sort_by_key(|b| -b.last_used.timestamp());
        branches.truncate(n);
        data.branches = Some(branches);
        self.write_to_json(&data)
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
