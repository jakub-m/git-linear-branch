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
        let info_list = self.storage.list_branch_info()?;
        for info in &info_list {
            println!("{:?}", info);
        }
        Ok(())
    }
}
