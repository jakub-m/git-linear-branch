use crate::command::Command;
use crate::command_push::sanitize_branch_name;
use crate::storage::{BranchInfo, Storage};

/// Format new branch, create the git branch, and update the storage.
pub struct NewBranchWithLastPrefix<'a> {
    storage: &'a dyn Storage,
    formatted_branch_name: String,
}

pub const COMMAND_NEW_BRANCH_WITH_LAST_PREFIX: &str = "new-branch-with-last-prefix";

const SEP: &str = "-";

impl<'a> NewBranchWithLastPrefix<'a> {
    /// Treat the first arg as a branch prefix.
    pub fn new(
        storage: &'a dyn Storage,
        args: &Vec<String>,
    ) -> Result<NewBranchWithLastPrefix<'a>, String> {
        let mut branches = storage.list_branch_info()?;
        if branches.is_empty() {
            return Err("there are no branches yet".to_string());
        }
        branches.sort_by_key(|v| -v.last_used.timestamp());
        let latest_branch = branches.get(0).unwrap();

        let mut args_to_format: Vec<&str> = vec![&latest_branch.prefix.as_str()];
        for arg in args {
            args_to_format.push(&arg.as_str());
        }
        let formatted_branch_name = args_to_format.join(SEP);
        Ok(NewBranchWithLastPrefix {
            storage,
            formatted_branch_name,
        })
    }
}

impl<'a> Command for NewBranchWithLastPrefix<'a> {
    fn run(&self) -> Result<(), String> {
        let b = &self.formatted_branch_name;
        print!("{b}");
        Ok(())
    }
}
