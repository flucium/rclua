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
    /// Lua 5.4.6
    V5_4_6,

    /// Lua 5.4.0
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

/// Lua obj.
#[derive(Debug)]
pub struct Lua(Version, std::ffi::CString);

impl Lua {
    /// Create a new Lua obj.
    ///
    /// # Arguments
    /// - `version` - The version of Lua to use for evaluating the code.
    pub fn new(version: Version) -> Self {
        Self(version, std::ffi::CString::default())
    }

    /// Load Lua code.
    ///
    /// # Arguments
    /// - `code` - A string slice containing the Lua code to be evaluated.
    pub fn load(
        &mut self,
        code: impl Into<String>,
    ) -> std::result::Result<&mut Self, std::ffi::NulError> {
        let code = code.into();

        let code = if code.ends_with('\0') {
            code.trim_end_matches('\0').to_string()
        } else {
            code
        };

        self.1 = std::ffi::CString::new(code)?;

        Ok(self)
    }

    /// Execute Lua code.
    pub fn execute(&self) {
        unsafe {
            match self.0 {
                Version::V5_4_6 => execute_5_4_6(self.1.as_ptr() as *const c_char),
                Version::V5_4_0 => execute_5_4_0(self.1.as_ptr() as *const c_char),
            }
        }
    }

    /// Execute Lua code.
    ///
    /// This is an alias for `execute`.
    pub fn exec(&self) {
        self.execute();
    }
    
    /// Get the version of Lua.
    pub fn version(&self) -> &Version {
        &self.0
    }

    /// Get the Lua code.
    pub fn code(&self) -> &std::ffi::CString {
        &self.1
    }
}

impl Default for Lua {
    /// Create a new Lua obj with default values.
    /// Lua version is 5.4.6
    fn default() -> Self {
        Self(Version::V5_4_6, std::ffi::CString::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_as_str() {
        assert_eq!(Version::V5_4_6.as_str(), "5.4.6");
        assert_eq!(Version::V5_4_0.as_str(), "5.4.0");
    }

    #[test]
    fn test_version_to_string() {
        assert_eq!(Version::V5_4_6.to_string(), "5.4.6");
        assert_eq!(Version::V5_4_0.to_string(), "5.4.0");
    }

    #[test]
    fn test_lua_5_4_0_load() {
        let mut lua = Lua::new(Version::V5_4_0);

        let is_ok = lua.load(
            r#"
        local count = 0

        function hanoi(n, a, b, c)
            if n > 0 then
                hanoi(n-1, a, c, b)
                
                count = count + 1
                
                hanoi(n-1, c, b, a)
            end
        end

        hanoi(3, 'a', 'b', 'c')
    "#,
        );

        assert!(is_ok.is_ok());
    }

    #[test]
    fn test_lua_5_4_6_load() {
        let mut lua = Lua::new(Version::V5_4_6);

        let is_ok = lua.load(
            r#"
        local count = 0

        function hanoi(n, a, b, c)
            if n > 0 then
                hanoi(n-1, a, c, b)
                
                count = count + 1
                
                hanoi(n-1, c, b, a)
            end
        end

        hanoi(3, 'a', 'b', 'c')
    "#,
        );

        assert!(is_ok.is_ok());
    }
}
