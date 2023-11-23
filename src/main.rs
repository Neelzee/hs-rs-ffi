use libloading;
use std::error::Error;
use std::os::raw::{c_int, c_char};

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let lib = libloading::Library::new("./mymodule.dll")?;
        //                                                    argc        argv
        let hs_init: libloading::Symbol<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char)> = lib.get(b"hs_init")?;
        let hs_exit: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"hs_exit")?;

        hs_init(std::ptr::null_mut(), std::ptr::null_mut()); // Initialize Haskell runtime
        let result = call_dynamic(&lib, 10);
        hs_exit(); // De-initialize Haskell runtime

        match result {
            Ok(e) => println!("Got this as result: {}", e),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

fn call_dynamic(lib: &libloading::Library, i: i32) -> Result<i32, Box<dyn Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(i32) -> i32> = lib.get(b"fibonacci")?;
        Ok(func(i))
    }
}
