use std::process;

use crate::command::Command;
use crate::storage::{BranchInfo, Storage};
use chrono::Utc;
use regex::Regex;

// TODO remove this command.
pub struct PushCommand<'a> {
    storage: &'a dyn Storage,
    branch_name: String,
}

/// Memorize a branch prefix.
impl<'a> PushCommand<'a> {
    pub fn from_args(
        args: &Vec<String>,
        storage: &'a dyn Storage,
    ) -> Result<PushCommand<'a>, String> {
        let args = PushCommandArgs::from_args(args)?;
        Ok(PushCommand {
            storage,
            branch_name: args.branch_name,
        })
    }
}

impl<'a> Command for PushCommand<'a> {
    fn run(&self) -> Result<(), String> {
        let prefix = sanitize_branch_name(&self.branch_name);
        let last_used = chrono::Utc::now();
        let info = BranchInfo {
            prefix,
            name: self.branch_name.to_owned(),
            last_used,
        };
        self.storage.store_branch_info(&info)?;
        Ok(())
    }
}

pub fn sanitize_branch_name(branch_name: &str) -> String {
    let pat = Regex::new(r"^([^\s]+/\w+-\d+)").unwrap();
    pat.find(branch_name)
        .map(|s| s.as_str())
        .unwrap_or(branch_name)
        .to_string()
}

#[derive(Debug)]
pub struct PushCommandArgs {
    pub branch_name: String,
}

impl PushCommandArgs {
    pub fn from_args(args: &Vec<String>) -> Result<PushCommandArgs, String> {
        let mut branch_name = None;
        let mut args_iter = args.iter();
        while let Some(arg) = args_iter.next() {
            if arg == "-h" {
                PushCommandArgs::print_help();
                process::exit(0);
            } else if branch_name.is_none() {
                branch_name = Some(arg.to_string());
            } else {
                return Err(format!("unknown param {:?}", arg));
            }
        }
        let branch_name = branch_name.ok_or("branch missing".to_string())?;
        Ok(PushCommandArgs { branch_name })
    }

    fn print_help() {
        let help = "
Pass branch name to push.
";
        println!("{}", help.trim())
    }
}
