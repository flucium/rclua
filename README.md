# RCLua
RCLua (rclua) is the name given to the combination of Rust, C, and Lua (official).

RCLua allows you to run Lua code from Rust. A key feature is that it wraps Lua official source code in a C program and calls it from Rust. This should make it easier to change Lua versions.

This is still in beta version, so we are not accepting pull requests at this time. if you have any feedback or requests, please contact me.

## Build

**Linux (Ubuntu)**

```bash
git clone git@github.com:flucium/rclua.git

cd ./rclua

# Release build
zsh ./build.sh release

# Debug
# zsh ./build.sh debug

# Package
# zsh ./build.sh package

# Cleanup
# zsh ./build.sh clean
```


## How to use

```bash
git clone git@github.com:flucium/rclua.git

cd ./rclua

# Release build
zsh ./build.sh release
```

```bash
cargo new app
```

```toml
[package]
name = "app"
version = "0.0.1"
edition = "2021"

[dependencies]
rclua = {path = "../rclua", version = "0.0.1"}
```

```rust
use rclua::{eval, Version};

fn main(){
    let code = "print('Hello, world!')\0";

    let version = Version::V5_4_6;

    eval(code, version);
}
```

# Lua
[https://www.lua.org](https://www.lua.org)

[https://www.lua.org/images/](https://www.lua.org/images/)

![Lua](./lua-logo.gif "Lua")