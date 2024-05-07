# RCLua
![Crates.io Version](https://img.shields.io/crates/v/rclua?style=flat&link=https%3A%2F%2Fcrates.io%2Fcrates%2Frclua)
![docs.rs](https://img.shields.io/docsrs/rclua?link=https%3A%2F%2Fdocs.rs%2Frclua%2F0.0.1%2Frclua%2F)
![GitHub License](https://img.shields.io/github/license/flucium/rclua?style=flat)

RCLua (rclua) is the name given to the combination of Rust, C, and Lua (official).

RCLua allows you to run Lua code from Rust. A key feature is that it wraps Lua official source code in a C program and calls it from Rust. This should make it easier to change Lua versions.

One of the key features of RCLua is that you can use Lua without installing it on your system. Even if Lua is already installed on the system, RCLua will not interfere with it. You can use it without polluting your system. This is because RCLua directly includes the official Lua source code and uses it as a library.

*this document assumes RCLua v0.0.1*

## Build
Rough environment: Rust 1.77.2 aarch64-apple-darwin / GCC 11.4.0 / CC 11.4.0 / GNU Make 4.3.

```bash
git clone git@github.com:flucium/rclua.git

# Release build
bash ./rclua/build.sh release
```

## Usage
There are two main ways to use it. 1: crates.io. 2: self build.

### crates.io
Please check version: [https://crates.io/crates/rclua](https://crates.io/crates/rclua)

```TOML
# Open `Cargo.toml` in any editor.
# add [dependencies] rclua = "0.0.1" to Cargo.toml.

[dependencies]
rclua = "0.0.1"
```

### self
Do the build as described in Section 'Build'.

```TOML
# Open `Cargo.toml` in any editor.
# add [dependencies] rclua = "0.0.1" to Cargo.toml.
# 'path = ...' must be the location of RCLua(rclua).

[dependencies]
rclua = { path = "../rclua", version = "0.0.1" }
```

## License
RCLua is licensed under the [MIT License](./LICENSE). 

For Third-party licenses please read [LICENSE_THIRDPARTY](./LICENSE_THIRDPARTY).

# Lua
[https://www.lua.org](https://www.lua.org)

[https://www.lua.org/images/](https://www.lua.org/images/)

![Lua](./lua-logo.gif "Lua")