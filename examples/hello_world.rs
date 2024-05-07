use rclua::{eval, Version};

fn main() {
    eval("print('Hello, World!')\0", Version::default());
}
