use std::env;
use std::path::Path;
use wolfu::coreutils::echo;
use wolfu::register_commands;

fn main() {
    let commands = register_commands! {
        "echo" => echo::run
    };

    let program_name = env::args().next().unwrap();
    let prog = Path::new(&program_name)
        .file_name()
        .unwrap()
        .to_string_lossy();

    match commands.get(prog.as_ref()) {
        Some(cmd) => cmd(),
        None => eprintln!("Unknown command: {}", prog),
    }
}
