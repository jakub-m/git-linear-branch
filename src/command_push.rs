use crate::storage::Storage;
pub struct PushCommand<'a> {
    storage: &'a dyn Storage,
}

/// Memorize a branch prefix.
impl<'a> PushCommand<'a> {
    pub fn new(storage: &'a dyn Storage) -> PushCommand {
        PushCommand { storage }
    }
    pub fn run(&self, branch_prefix: &str) -> Result<(), String> {
        // sanitize branch prefix
        self.storage.store_branch_prefix(branch_prefix)
    }
}
