fn main() {
    std::fs::read_to_string("./examples/script.lua")
        .map(|script| rclua::eval(&script, rclua::Version::default()))
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
        });
}
