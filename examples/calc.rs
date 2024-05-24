use rclua::*;

const LUA_SCRIPT_PLUS: &str = "print ( 1 + 2 )";

const LUA_SCRIPT_MINUS: &str = "print ( 1 - 2 )";

const LUA_SCRIPT_MULTIPLY: &str = "print ( 1 * 2 )";

const LUA_SCRIPT_DIVIDE: &str = "print ( 1 / 2 )";

const LUA_SCRIPT_PLUS_FUNCTION: &str = "function add(a, b) return a + b end print(add(1, 2))";

const LUA_SCRIPT_MINUS_FUNCTION: &str =
    "function subtract(a, b) return a - b end print(subtract(1, 2))";

const LUA_SCRIPT_MULTIPLY_FUNCTION: &str =
    "function multiply(a, b) return a * b end print(multiply(1, 2))";

const LUA_SCRIPT_DIVIDE_FUNCTION: &str =
    "function divide(a, b) return a / b end print(divide(1, 2))";

fn main() {
    execute(LUA_SCRIPT_PLUS, Version::V5_4_6);

    execute(LUA_SCRIPT_MINUS, Version::V5_4_6);

    execute(LUA_SCRIPT_MULTIPLY, Version::V5_4_6);

    execute(LUA_SCRIPT_DIVIDE, Version::V5_4_6);

    execute(LUA_SCRIPT_PLUS_FUNCTION, Version::V5_4_6);

    execute(LUA_SCRIPT_MINUS_FUNCTION, Version::V5_4_6);

    execute(LUA_SCRIPT_MULTIPLY_FUNCTION, Version::V5_4_6);

    execute(LUA_SCRIPT_DIVIDE_FUNCTION, Version::V5_4_6);
}
