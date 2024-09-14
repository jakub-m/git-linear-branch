use crate::command_list::COMMAND_LIST;
use crate::command_new_branch::COMMAND_BRANCH;
use crate::command_new_branch_with_last_prefix::COMMAND_NEW_BRANCH_WITH_LAST_PREFIX;
use std::process;

#[derive(Debug)]
pub struct Args {
    /// The matched command,
    // pub command: Option<String>,
    /// Remaining unparsed args
    pub args: Vec<String>,
}

const LEGAL_COMMANDS: &'static [&'static str] = &[
    COMMAND_BRANCH,
    COMMAND_LIST,
    COMMAND_NEW_BRANCH_WITH_LAST_PREFIX,
];

impl Args {
    pub fn from_args(args: &Vec<String>) -> Result<Args, String> {
        let mut output = Args {
            // command: None,
            args: vec![],
        };
        let mut iter_args = args.iter();
        iter_args.next();

        while let Some(arg) = iter_args.next() {
            if arg == "-h" {
                Args::print_help();
                process::exit(0);
            //} else if output.command.is_none() && is_legal_command(arg) {
            //    output = Args {
            //        command: Some(arg.to_owned()),
            //        args: iter_args.map(|s| s.to_owned()).collect(),
            //        ..output
            //    };
            //    return Ok(output);
            } else {
                let mut args = vec![arg.to_owned()];
                for arg in iter_args {
                    args.push(arg.to_owned());
                }
                output = Args { args, ..output };
                return Ok(output);
            }
        }
        return Ok(output); // No command.
    }

    fn print_help() {
        let help = "
Utility to create branch names from past linear branch names.
Commands
\tbranch (default)
\tlist

-h\tThis help.
";
        println!("{}", help.trim())
    }
}

fn is_legal_command(s: &str) -> bool {
    LEGAL_COMMANDS.contains(&s)
}
