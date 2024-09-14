use std::process;

#[derive(Debug)]
pub struct Args {
    /// The matched command,
    // pub command: Option<String>,
    /// Remaining unparsed args
    pub args: Vec<String>,
}

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
-h\tThis help.
";
        println!("{}", help.trim())
    }
}
