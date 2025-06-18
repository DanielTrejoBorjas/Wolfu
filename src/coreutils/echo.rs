use std::env;
use std::io::{self, Write};

pub fn run() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut no_newline = false;

    if let Some(first) = args.first() {
        if first == "-n" {
            no_newline = true;
            args.remove(0); // Quitamos "-n"
        }
    }

    print!("{}", args.join(" "));
    
    if no_newline {
        io::stdout().flush().unwrap();
    } else {
        println!();
    }
}
