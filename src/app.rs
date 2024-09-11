use std::env;

use crate::args::Args;
use crate::command::Command;
use crate::command_list::{ListCommand, COMMAND_LIST};
use crate::command_new_branch::{NewBranch, COMMAND_BRANCH};
use crate::storage::JsonStorage;

const DEFAULT_STORAGE_FILENAME: &str = ".meta-for-git.json";

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage::new(DEFAULT_STORAGE_FILENAME)?;
    let command_str = args.command.unwrap_or(COMMAND_BRANCH.to_string());
    dbg!(&command_str);
    let command: Box<dyn Command> = match command_str.as_ref() {
        COMMAND_LIST => Box::new(ListCommand::new(&storage)?),
        COMMAND_BRANCH => Box::new(NewBranch::new(&storage, &args.args)?),
        _ => {
            todo!()
        }
    };
    command.run()
}
