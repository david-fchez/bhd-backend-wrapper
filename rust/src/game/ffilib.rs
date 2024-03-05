mod ffi;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
extern crate libloading as lib;
use gdnative::api::*;
use gdnative::prelude::*;
/*
List of targets:
"windows"
"macos"
"ios"
"linux"
"android"
"freebsd"
"dragonfly"
"openbsd"
"netbsd"
*/

fn lib_file() -> String {
    #[cfg(target_os = "linux")]
    {
        String::from("./libs/linux64/libbhd.so")
    }
    #[cfg(target_os = "macos")]
    {
        String::from("@executable_path/../Frameworks/libbhd.dylib")
    }
    #[cfg(target_os = "windows")]
    {
        String::from("libs/windows64/libbhd.dll")
    }
    #[cfg(target_os = "android")]
    {
        String::from("libbhd.so")
    }
    #[cfg(target_os = "ios")]
    {
        String::from("@loader_path/../Frameworks/libbhd.dylib")
    }
}

pub struct FfiWrapper {
    golang_lib: lib::Library,
}

impl FfiWrapper{
    //load golang library
    pub fn new() -> Self {
        unsafe{
            let lib = lib::Library::new(lib_file());
             FfiWrapper {
                golang_lib: lib.unwrap()
            }
        }
        
    }


    pub fn go_method_call(&mut self, method_name: String, data: String) -> Result<String, String> {
        //godot_print!("YO this is RUST. Calling golang method with method_name: {} and data: {}" , method_name.to_owned(), data.to_owned());
        let method_c_ref = CString::new(method_name).unwrap();
        let go_str_ref = ffi::GoString {
            p: method_c_ref.as_ptr(),
            n: method_c_ref.as_bytes().len() as isize,
        };

        let data_c_ref = CString::new(data).unwrap();
        let data_go_str_ref = ffi::GoString {
            p: data_c_ref.as_ptr(),
            n: data_c_ref.as_bytes().len() as isize,
        };

        unsafe {
            let go_func: lib::Symbol<unsafe extern "C" fn(method_name: ffi::GoString, data: ffi::GoString) -> *const c_char> = self.golang_lib.get(b"Call").unwrap();
            let result = go_func(go_str_ref, data_go_str_ref);
            let c_str =  CStr::from_ptr(result);
            let string = c_str.to_str().expect("Error");
            
            //godot_print!("Hello this is Rust (again) golang returned: {}" , string.to_owned());

            match string.is_empty() {
                true =>  Err("golang didn't return a result".to_owned()),
                false => Ok(string.to_owned()),
            }
        }
    }

}


