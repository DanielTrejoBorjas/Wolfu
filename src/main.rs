use std::env;
use std::path::Path;
use wolfu::coreutils::{echo}; 

macro_rules! register_commands {
    ( $( $name:literal => $path:path ),* $(,)? ) => {{
        let mut map: std::collections::HashMap<&'static str, fn()> = std::collections::HashMap::new();
        $(
            map.insert($name, $path);
        )*
        map
    }};
}

fn main() {
    let commands = register_commands! {
        "echo" => echo::run
    };

    let program_name = env::args().next().unwrap();
    let prog = Path::new(&program_name)
        .file_name().unwrap().to_string_lossy();

    match commands.get(prog.as_ref()) {
        Some(cmd) => cmd(),
        None => eprintln!("Unknown command: {}", prog),
    }
}
