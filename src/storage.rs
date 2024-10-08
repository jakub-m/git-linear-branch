use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use std::{fs::File, io::Read};

#[derive(serde::Serialize, serde::Deserialize)]
struct Stored {
    branches: Option<Vec<BranchInfo>>,
}

pub trait Storage {
    fn push_branch_info(&self, info: &BranchInfo) -> Result<(), StorageError>;
    fn replace_branch_info(&self, info: &Vec<BranchInfo>) -> Result<(), StorageError>;
    fn list_branch_info(&self) -> Result<Vec<BranchInfo>, StorageError>;
    fn trim_to_latest(&self, n: usize) -> Result<(), StorageError>;
    fn get_by_prefix(&self, prefix: &str) -> Result<Option<BranchInfo>, StorageError>;
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BranchInfo {
    /// Branch prefix.
    pub prefix: String,
    /// Full branch name.
    pub name: String,
    #[serde(with = "ts_seconds")]
    pub last_used: DateTime<Utc>,
    /// A readable title describing the original branch.
    pub original_title: String,
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
        Ok(JsonStorage {
            filepath: filepath.to_owned(),
        })
    }

    fn read_from_json(&self) -> Result<Stored, StorageError> {
        let mut file = match File::open(&self.filepath) {
            Ok(file) => file,
            Err(_) => return Ok(Stored { branches: None }),
        };
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
    /// Prefix is the key. If there is a branch with the same prefix, then replace the existing
    /// branch with the new branch.
    fn push_branch_info(&self, info: &BranchInfo) -> Result<(), StorageError> {
        let mut data = self.read_from_json()?;
        let branches = data.branches.take().unwrap_or(vec![]);

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

    fn replace_branch_info(&self, info: &Vec<BranchInfo>) -> Result<(), StorageError> {
        let branches: Vec<BranchInfo> = info.iter().map(|b| b.to_owned()).collect();
        let data = Stored {
            branches: Some(branches),
        };
        self.write_to_json(&data)?;
        Ok(())
    }

    fn list_branch_info(&self) -> Result<Vec<BranchInfo>, StorageError> {
        let data = self.read_from_json()?;
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

    fn get_by_prefix(&self, prefix: &str) -> Result<Option<BranchInfo>, StorageError> {
        let data = self.read_from_json()?;
        if let Some(branches) = data.branches {
            Ok(branches
                .iter()
                .find(|p| p.prefix == prefix)
                .map(|b| b.clone()))
        } else {
            Ok(None)
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
