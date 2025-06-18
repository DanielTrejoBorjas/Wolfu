use wolfu::coreutils::echo::Echo;
use wolfu::coreutils::cat::Cat;
use wolfu::register_commands;
fn main() {
    let commands = register_commands! {
        "echo" => Echo,
        "cat" => Cat
    };

    let args: Vec<String> = std::env::args().collect();
    let prog = std::path::Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_string_lossy();

    if let Some(cmd) = commands.get(prog.as_ref()) {
        if let Err(e) = cmd.run(&args[1..]) {
            eprintln!("Error executing command '{}': {}", cmd.name(), e);
        }
    } else {
        eprintln!("Unknown command: {}", prog);
    }
}
