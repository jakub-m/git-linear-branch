use std::env;

use crate::args::Args;
use crate::command::Command;
use crate::command_list::ListCommand;
use crate::command_push::PushCommand;
use crate::storage::JsonStorage;

const DEFAULT_STORAGE_FILENAME: &str = ".meta-for-git.json";

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage::new(DEFAULT_STORAGE_FILENAME)?;
    let command_str = args.command.expect("command missing");
    let command: Box<dyn Command> = match command_str.as_ref() {
        "push" => Box::new(PushCommand::from_args(&args.args, &storage)?),
        "list" => Box::new(ListCommand::new(&storage)?),
        _ => {
            todo!()
        }
    };
    command.run()
}
