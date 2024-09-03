use std::process;

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

#[derive(Debug)]
pub struct PushCommandArgs {
    pub branch_name: Option<String>,
}

impl PushCommandArgs {
    pub fn from_args(args: &Vec<String>) -> Result<PushCommandArgs, String> {
        let mut output = PushCommandArgs { branch_name: None };
        let mut iter_args = args.iter();
        iter_args.next();

        while let Some(arg) = iter_args.next() {
            if arg == "-h" {
                PushCommandArgs::print_help();
                process::exit(0);
            } else if output.branch_name.is_none() {
                output = PushCommandArgs {
                    branch_name: Some(arg.to_string()),
                    ..output
                }
            } else {
                return Err(format!("unknown param {:?}", arg));
            }
        }
        if output.branch_name.is_none() {
            return Err("branch missing".to_string());
        }
        Ok(output)
    }

    fn print_help() {
        let help = "
Pass branch name to push.
";
        println!("{}", help.trim())
    }
}
