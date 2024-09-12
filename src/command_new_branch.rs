use crate::command::Command;
use crate::command_push::sanitize_branch_name;
use crate::storage::{BranchInfo, Storage};

/// Format new branch, create the git branch, and update the storage.
pub struct NewBranch<'a> {
    storage: &'a dyn Storage,
    prefix: String,
    formatted_branch_name: String,
}

pub const COMMAND_BRANCH: &str = "branch";
const SEP: &str = "-";

impl<'a> NewBranch<'a> {
    /// Treat the first arg as a branch prefix.
    pub fn new(storage: &'a dyn Storage, args: &Vec<String>) -> Result<NewBranch<'a>, String> {
        let prefix: String;
        if let Some(arg_prefix) = args.get(0) {
            prefix = sanitize_branch_name(&arg_prefix);
        } else {
            return Err("no arguments for branch".to_string());
        }

        let mut args_to_format: Vec<String> = vec![prefix.to_owned()];
        args_to_format.extend_from_slice(&args[1..]);
        let formatted_branch_name = args_to_format.join(SEP);

        Ok(NewBranch {
            storage,
            prefix,
            formatted_branch_name,
        })
    }
}

impl<'a> Command for NewBranch<'a> {
    fn run(&self) -> Result<(), String> {
        let info = BranchInfo {
            prefix: self.prefix.to_owned(),
            name: self.formatted_branch_name.to_owned(),
        };
        self.storage.store_branch_info(&info)?;
        let formatted = &self.formatted_branch_name;
        print!("{formatted}");
        Ok(())
    }
}
