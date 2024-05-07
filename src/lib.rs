extern crate libc;
use libc::c_char;

#[link(name = "clua", kind = "static")]
extern "C" {
    pub fn eval(code: *const c_char);
}

pub mod lua {
    use super::{eval as eval_, *};

    pub unsafe fn eval(code: &str) {
        unsafe {
            eval_(code.as_ptr() as *const c_char);
        }
    }

    #[test]
    fn test_eval() {
        unsafe {
            eval("print('Hello, World!')");
        }
    }
}

#[test]
fn test_eval() {
    unsafe {
        eval("print('Hello, World!')\0".as_ptr() as *const c_char);
    }
}
