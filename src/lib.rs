pub extern crate libc;
use libc::c_char;

#[link(name = "clua_5_4_6", kind = "static")]
extern "C" {
    /// Evaluate Lua code for Lua 5.4.6
    fn execute_5_4_6(code: *const c_char);
}

#[link(name = "clua_5_4_0", kind = "static")]
extern "C" {
    /// Evaluate Lua code for Lua 5.4.0
    fn execute_5_4_0(code: *const c_char);
}

fn _execute_5_4_6(code: &str) {
    unsafe {
        execute_5_4_6(code.as_ptr() as *const c_char);
    }
}

fn _execute_5_4_0(code: &str) {
    unsafe {
        execute_5_4_0(code.as_ptr() as *const c_char);
    }
}

/// Lua version
///
/// Default version is 5.4.6
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Version {
    #[default]
    V5_4_6,
    V5_4_0,
}

impl Version {
    /// Get the version as a &str (e.g. "5.4.6")
    pub fn as_str(&self) -> &str {
        match self {
            Version::V5_4_6 => "5.4.6",
            Version::V5_4_0 => "5.4.0",
        }
    }
}

impl ToString for Version {
    /// Get the version as a String (e.g. "5.4.6")
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// Evaluate Lua code.
///
/// This function takes a string of Lua code and a version of Lua, and evaluates the code using the specified version of Lua.
///
/// # Arguments
///
/// * `code` - A string slice containing the Lua code to be evaluated.
/// * `version` - The version of Lua to use for evaluating the code.
///
/// # Example
///
/// ```
/// use rclua::{execute, Version};
///
/// let code = "print('Hello, world!')\0";
///
/// let version = Version::V5_4_6;
///
/// execute(code, version);
///
/// ```
pub fn execute(code: &str, version: Version) {
    let code = code.to_string();

    let code = if code.ends_with('\0') {
        code
    } else {
        code + "\0"
    };

    unsafe {
        match version {
            Version::V5_4_6 => execute_5_4_6(code.as_ptr() as *const c_char),
            Version::V5_4_0 => execute_5_4_0(code.as_ptr() as *const c_char),
        }
    }
}

#[test]
fn test_execute_5_4_6() {
    unsafe {
        execute_5_4_6("print('Hello, Lua 5.4.6!')\0".as_ptr() as *const c_char);
    }
}

#[test]
fn test_execute_5_4_0() {
    unsafe {
        execute_5_4_0("print('Hello, Lua 5.4.0!')\0".as_ptr() as *const c_char);
    }
}

#[test]
fn test_eval() {
    execute("print('Hello, world!')\0", Version::default());

    execute("print('Hello, world!')\0", Version::V5_4_6);

    execute("print('Hello, world!')\0", Version::V5_4_0);
}

#[test]
fn test_eval_not_eof() {
    execute("print('Hello, world!')", Version::default());

    execute("print('Hello, world!')", Version::V5_4_6);
}
