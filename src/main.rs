use wolfu::coreutils::command::ExitResult;
use wolfu::register_commands;
fn main() {
    // Register commands
    let commands = register_commands! {
        "echo" => wolfu::coreutils::echo::Echo,
        "cat" => wolfu::coreutils::cat::Cat,
        "true" => wolfu::coreutils::true_::True,
        "false" => wolfu::coreutils::false_::False,
    };

    // Get command-line arguments
    // and execute the command
    let args: Vec<String> = std::env::args().collect();
    let prog = std::path::Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_string_lossy();

    if let Some(cmd) = commands.get(prog.as_ref()) {
        match cmd.run(&args[1..]) {
            ExitResult::Exit(code) => std::process::exit(code),
            ExitResult::Continue(result) => {
                if let Err(e) = result {
                    eprintln!("Error executing command '{}': {}", cmd.name(), e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Unknown command: {}", prog);
        std::process::exit(0);
    }
}
