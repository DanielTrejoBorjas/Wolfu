use crate::coreutils::command::{Command, ExitResult};
use std::fs::File;
use std::io::{self, Read};

pub struct Cat;

impl Command for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn run(&self, args: &[String]) -> ExitResult {
        if args.is_empty() {
            let mut stdin = io::stdin();
            let mut buf = String::new();
            if let Err(e) = stdin.read_to_string(&mut buf) {
                return ExitResult::Continue(Err(e.to_string()));
            }
            print!("{}", buf);
            return ExitResult::Continue(Ok(()));
        }

        for filename in args {
            let mut file = match File::open(filename) {
                Ok(f) => f,
                Err(e) => return ExitResult::Continue(Err(format!("{}: {}", filename, e))),
            };
            let mut buf = String::new();
            if let Err(e) = file.read_to_string(&mut buf) {
                return ExitResult::Continue(Err(format!("{}: {}", filename, e)));
            }
            print!("{}", buf);
        }

        ExitResult::Continue(Ok(()))
    }

    fn help(&self) -> &'static str {
        "Usage: cat [FILE...]\nConcatenate FILE(s), or standard input, to standard output."
    }
}
