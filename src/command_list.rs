use crate::{command::Command, storage::Storage};

pub const COMMAND_LIST: &str = "list";

pub struct ListCommand<'a> {
    storage: &'a dyn Storage,
}

impl<'a> ListCommand<'a> {
    pub fn new(
        storage: &'a dyn Storage,
        // output:  TODO add output writer
    ) -> Result<ListCommand<'a>, String> {
        Ok(ListCommand { storage })
    }
}

impl<'a> Command for ListCommand<'a> {
    fn run(&self) -> Result<(), String> {
        let prefixes = self.storage.list_prefixes()?;
        for prefix in prefixes {
            println!("{}", prefix);
        }
        Ok(())
    }
}
