use crate::{command::Command, storage::Storage};
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
        todo!()
    }
}
