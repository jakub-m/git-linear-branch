use crate::storage::Storage;
use regex::Regex;
pub struct PushCommand<'a> {
    storage: &'a dyn Storage,
}

/// Memorize a branch prefix.
impl<'a> PushCommand<'a> {
    pub fn new(storage: &'a dyn Storage) -> PushCommand {
        PushCommand { storage }
    }
    pub fn run(&self, branch_prefix: &str) -> Result<(), String> {
        let branch_prefix = sanitize_branch_name(branch_prefix);
        self.storage.store_branch_prefix(&branch_prefix)
    }
}

fn sanitize_branch_name(branch_name: &str) -> String {
    let pat = Regex::new(r"^([^\s]+/\w+-\d+)").unwrap();
    pat.find(branch_name)
        .map(|s| s.as_str())
        .unwrap_or(branch_name)
        .to_string()
}
