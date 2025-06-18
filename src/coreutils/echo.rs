use std::io::{self, Write};
use crate::coreutils::command::Command;

pub struct Echo;

// Implements the Command trait for the Echo command
// This command prints its arguments to standard output, with an option to suppress the trailing newline.
impl Command for Echo {
    
    // Returns the name of the command
    // This is used to register the command and identify it in the command map.
    // In this case, the command is named "echo".
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[String]) -> Result<(), String> {
        let mut args = args.to_vec();
        let mut no_newline = false;

        if let Some(first) = args.first() {
            if first == "-n" {
                no_newline = true;
                args.remove(0);
            }
        }

        print!("{}", args.join(" "));
        if no_newline {
            io::stdout().flush().map_err(|e| e.to_string())?;
        } else {
            println!();
        }
        Ok(())
    }

    fn help(&self) -> &'static str {
        "Usage: echo [-n] [string...]\nPrint the STRING(s) to standard output."
    }
}
