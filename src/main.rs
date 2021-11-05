
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use root::LuauWrapper;
use std::ffi::CString;

impl Drop for LuauWrapper {
    fn drop(&mut self) {
        unsafe {
            self.destruct();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let name = CString::new("demo.luau").unwrap();

    unsafe {
        let mut luau = LuauWrapper::new();
        luau.check(name.as_ptr());
    }
}
