pub trait Storage {
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), String>;
}
pub struct JsonStorage {}

impl Storage for JsonStorage {
    fn store_branch_prefix(&self, branch_prefix: &str) -> Result<(), String> {
        dbg!(branch_prefix);
        Ok(())
    }
}
