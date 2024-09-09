use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Stored {
    branches: Option<Vec<String>>,
}

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
        let mut json_data: Stored = serde_json::from_str(&content)
            .map_err(|err| format!("storage file is not valid JSON {}: {}", self.filepath, err))?;

        let mut branches = json_data.branches.take().unwrap_or(vec![]);
        if branches.iter().find(|s| *s == branch_prefix).is_some() {
            return Ok(());
        }

        branches.push(branch_prefix.to_owned());
        json_data.branches = Some(branches);

        let file = File::create(&self.filepath)
            .map_err(|err| format!("failed to open {} to write: {}", self.filepath, err))?;
        serde_json::to_writer_pretty(file, &json_data)
            .map_err(|err| format!("failed to write {}: {}", self.filepath, err))
    }
}
