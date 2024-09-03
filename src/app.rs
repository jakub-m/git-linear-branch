use std::env;

use crate::args::{Args, Command};
use crate::command_push::PushCommand;

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    match args.command.as_ref().expect("bug: command missing") {
        Command::PushPrefix => {
            let c = PushCommand::new();
            c.run();
        }
    };
    dbg!(args);
    Ok(())
}
