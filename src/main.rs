use std::process;

mod app;
mod args;
mod command;
mod command_list;
mod command_new_branch;
mod command_new_branch_with_last_prefix;
mod command_push;
mod storage;

fn main() {
    if let Err(error) = app::run() {
        println!("ERROR: {}", error);
        process::exit(1)
    }
}
