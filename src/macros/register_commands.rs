/* Register command in hashmap */
// I don't understand this code, it's the result of a night when I had nothing to do while
// I was stressed watching how to solve the problem, I don't even know how I came up with the
// idea of ​​using a macro, but it works.
#[macro_export]
macro_rules! register_commands {
    ( $( $name:literal => $path:path ),* $(,)? ) => {{
        let mut map: std::collections::HashMap<&'static str, fn()> = std::collections::HashMap::new();
        $(
            map.insert($name, $path);
        )*
        map
    }};
}
