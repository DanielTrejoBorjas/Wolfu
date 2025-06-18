use crate::coreutils::command::{Command, ExitResult};

pub struct True;

impl Command for True {
    fn name(&self) -> &'static str {
        "true"
    }

    fn run(&self, _args: &[String]) -> ExitResult {
        ExitResult::Exit(0) // Always exit with success
    }

    fn help(&self) -> &'static str {
        "Usage: true\nDo nothing, successfully."
    }
}
