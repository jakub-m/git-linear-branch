use std::env;

use crate::args::Args;
use crate::command::Command;
use crate::command_format::FormatCommand;
use crate::command_list::ListCommand;
use crate::command_push::PushCommand;
use crate::storage::JsonStorage;

const DEFAULT_STORAGE_FILENAME: &str = ".meta-for-git.json";

const COMMAND_PUSH: &str = "push";
const COMMAND_LIST: &str = "list";
const COMMAND_FORMAT: &str = "format";

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage::new(DEFAULT_STORAGE_FILENAME)?;
    let command_str = args.command.unwrap_or(COMMAND_FORMAT.to_string());
    let command: Box<dyn Command> = match command_str.as_ref() {
        COMMAND_PUSH => Box::new(PushCommand::from_args(&args.args, &storage)?),
        COMMAND_LIST => Box::new(ListCommand::new(&storage)?),
        COMMAND_FORMAT => Box::new(FormatCommand::new(&args.args)),
        _ => {
            todo!()
        }
    };
    command.run()
}
