use std::env;

use crate::args::Args;
use crate::command_list::ListCommand;
use crate::command_push::PushCommand;
use crate::storage::JsonStorage;

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage::new("tmp.json")?;
    let command_str = args.command.expect("command missing");
    let command = match command_str.as_ref() {
        "push" => PushCommand::from_args(&args.args, &storage),
        "list" => ListCommand::new(&storage),
        // "list-prefixes" =>
        _ => {
            todo!()
        }
    }?;
    command.run()
}
