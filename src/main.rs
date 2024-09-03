use std::process;

mod app;
mod args;
mod command_push;
mod storage;

fn main() {
    if let Err(error) = app::run() {
        println!("ERROR: {}", error);
        process::exit(1)
    }
}
