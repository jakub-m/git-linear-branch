use crate::command::Command;
pub struct FormatCommand {
    formatted: String,
}

impl FormatCommand {
    pub fn new(args: &Vec<String>) -> FormatCommand {
        let formatted = args.join("-");
        FormatCommand { formatted }
    }
}

impl<'a> Command for FormatCommand {
    fn run(&self) -> Result<(), String> {
        let formatted = &self.formatted;
        print!("{formatted}");
        Ok(())
    }
}
