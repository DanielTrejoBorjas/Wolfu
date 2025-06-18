use crate::coreutils::command::Command;
use std::fs::File;
use std::io::{self, Read};

pub struct Cat;

impl Command for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn run(&self, args: &[String]) -> Result<(), String> {
        if args.is_empty() {
            // Leer desde stdin
            let mut stdin = io::stdin();
            let mut buf = String::new();
            stdin.read_to_string(&mut buf).map_err(|e| e.to_string())?;
            print!("{}", buf);
            return Ok(());
        }

        for filename in args {
            let mut file = File::open(filename).map_err(|e| format!("{}: {}", filename, e))?;
            let mut buf = String::new();
            file.read_to_string(&mut buf).map_err(|e| format!("{}: {}", filename, e))?;
            print!("{}", buf);
        }

        Ok(())
    }

    fn help(&self) -> &'static str {
        "Usage: cat [FILE...]\nConcatenate FILE(s), or standard input, to standard output."
    }
}
