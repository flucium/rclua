fn main() {
    std::fs::read_to_string("./examples/hanoi.lua")
        .map(|script| rclua::execute(&script, rclua::Version::V5_4_6))
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
        });
}
