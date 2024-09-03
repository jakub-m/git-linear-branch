use std::{env, process};

pub fn run() -> Result<(), String> {
    parse_cli_args()?;
    Ok(())
}

struct Args {}

fn parse_cli_args() -> Result<Args, String> {
    let mut output = Args {};
    let args: Vec<String> = env::args().collect();
    let mut iter_args = args.iter();
    iter_args.next();
    while let Some(arg) = iter_args.next() {
        if arg == "-h" {
            print_help();
            process::exit(0);
        }
        //        if found_sentinel {
        //            output = Args {
        //                expression: Some(output.expression.map_or(arg.to_owned(), |s| s + " " + arg)),
        //                ..output
        //            }
        //        } else {
        //            return Err(format!("unknown param {:?}", arg));
        //        }
        //    }
    }
    Ok(output)
}

fn print_help() {
    let help = "
Utility to create branch names from past linear branch names.

-h\tThis help.
";
    println!("{}", help.trim())
}
