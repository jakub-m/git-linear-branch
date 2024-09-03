use std::env;

use crate::args::{Args, Command};
use crate::command_push::PushCommand;
use crate::storage::JsonStorage;

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage {};
    match args.command.as_ref().expect("bug: command missing") {
        Command::PushPrefix => {
            let c = PushCommand::new(&storage);
            c.run("foomisfs-sdf/bar-123-goosdfs")?;
        }
    };
    dbg!(args);
    Ok(())
}
