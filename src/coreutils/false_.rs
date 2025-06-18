use crate::coreutils::command::{Command, ExitResult};

pub struct False;

impl Command for False {
    fn name(&self) -> &'static str {
        "false"
    }

    fn run(&self, _args: &[String]) -> ExitResult {
        ExitResult::Exit(1) 
    }

    fn help(&self) -> &'static str {
        "Usage: true\nDo nothing, successfully."
    }
}
