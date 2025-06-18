#[macro_export]
macro_rules! register_commands {
    ( $( $name:literal => $cmd:expr ),* $(,)? ) => {{
        let mut map: std::collections::HashMap<&'static str, Box<dyn $crate::coreutils::command::Command>> = std::collections::HashMap::new();
        $(
            map.insert($name, Box::new($cmd));
        )*
        map
    }};
}
