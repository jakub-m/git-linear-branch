pub trait Command {
    fn run(&self) -> Result<(), String>;
}
