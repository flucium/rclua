use rclua::*;

fn main() {
    eval("print('Hello, World!')", Version::V5_4_6);
}
