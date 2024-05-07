extern crate libc;
use libc::c_char;

#[link(name = "clua", kind = "static")]
extern "C" {
    /// Evaluate Lua code for Lua 5.4.6
    fn eval_5_4_6(code: *const c_char);
}

/// Lua version
///
/// Default version is 5.4.6
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Version {
    #[default]
    V5_4_6,
}

impl Version {
    /// Get the version as a &str (e.g. "5.4.6")
    pub fn as_str(&self) -> &str {
        match self {
            Version::V5_4_6 => "5.4.6",
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
/// use rclua::{eval, Version};
///
/// let code = "print('Hello, world!')\0";
///
/// let version = Version::V5_4_6;
///
/// eval(code, version);
///
/// ```
pub fn eval(code: &str, version: Version) {
    unsafe {
        match version {
            Version::V5_4_6 => eval_5_4_6(code.as_ptr() as *const c_char),
        }
    }
}

#[test]
fn test_version() {
    assert_eq!(Version::V5_4_6.as_str(), "5.4.6");

    assert_eq!(Version::V5_4_6.to_string(), "5.4.6");

    assert_eq!(Version::default(), Version::V5_4_6);
}

#[test]
fn test_eval() {
    eval("print('Hello, world!')\0", Version::default());

    eval("print('Hello, world!')\0", Version::V5_4_6);
}

#[test]
fn test_clua_eval_5_4_6() {
    unsafe {
        eval_5_4_6("print('Hello, world!')\0".as_ptr() as *const c_char);
    }
}
