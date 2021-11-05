
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

pub struct TypeChecker {
    p: root::LuauWrapper
}

impl TypeChecker {
    pub fn new() -> Self {
        unsafe {
            TypeChecker{p: root::LuauWrapper::new()}
        }
    }

    pub fn check(&mut self, filename: &str) {
        let cstr = CString::new(filename).expect("CString::new can fail?  What?");
        unsafe {
            self.p.check(cstr.as_ptr());
        }
    }
}

impl Drop for TypeChecker {
    fn drop(&mut self) {
        unsafe {
            self.p.destruct();
        }
    }
}
