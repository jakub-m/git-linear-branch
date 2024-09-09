use std::process;

#[derive(Debug)]
pub struct Args {
    /// The matched command,
    pub command: Option<String>,
    /// Remaining unparsed args
    pub args: Vec<String>,
}

impl Args {
    pub fn from_args(args: &Vec<String>) -> Result<Args, String> {
        let mut output = Args {
            command: None,
            args: vec![],
        };
        let mut iter_args = args.iter();
        iter_args.next();

        while let Some(arg) = iter_args.next() {
            if arg == "-h" {
                Args::print_help();
                process::exit(0);
            } else if output.command.is_none() {
                output = Args {
                    command: Some(arg.to_owned()),
                    args: iter_args.map(|s| s.to_owned()).collect(),
                    ..output
                };
                return Ok(output);
            } else {
                return Err(format!("unknown param {:?}", arg));
            }
        }
        return Err("command missing".to_string());
    }

    fn print_help() {
        let help = "
Utility to create branch names from past linear branch names.
Commands
\tpush
\tlist

-h\tThis help.
";
        println!("{}", help.trim())
    }
}
