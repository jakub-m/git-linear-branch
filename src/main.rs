use std::process;

mod app;
mod args;
mod command_push;

fn main() {
    if let Err(error) = app::run() {
        println!("ERROR: {}", error);
        process::exit(1)
    }
}
