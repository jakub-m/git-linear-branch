use std::process;

#[derive(Debug)]
pub struct Args {
    pub command: Option<Command>,
}

#[derive(Debug)]
pub enum Command {
    PushPrefix,
}

impl Args {
    pub fn from_args(args: &Vec<String>) -> Result<Args, String> {
        let mut output = Args { command: None };
        let mut iter_args = args.iter();
        iter_args.next();

        while let Some(arg) = iter_args.next() {
            if arg == "-h" {
                Args::print_help();
                process::exit(0);
            } else if output.command.is_none() {
                let command = match arg.as_str() {
                    "push" => Command::PushPrefix,
                    other => return Err(format!("unknown command: {}", other)),
                };
                output = Args {
                    command: Some(command),
                    ..output
                }
            } else {
                return Err(format!("unknown param {:?}", arg));
            }
        }
        if output.command.is_none() {
            return Err("command missing".to_string());
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
}
