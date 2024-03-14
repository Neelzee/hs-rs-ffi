use eyre::{Context, Result};
use libloading;
use std::os::raw::{c_char, c_int};

pub mod func;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let lib =
            unsafe { libloading::Library::new("./haskell/Foo.dll").expect("Should have lib") };

        match get_function::<i32, i32>(&lib, "foobar".to_string()) {
            Ok(func) => {
                unsafe {
                    start_hs(&lib).expect("Failed starting Haskell runtime");
                }
                let res = unsafe { func(10, 10) };
                unsafe {
                    end_hs(&lib).expect("Failed exiting Haskell runetime");
                }
                assert_eq!(res, 10 * 10)
            }
            Err(err) => {
                eprintln!("{:?}", err);
                assert!(false);
            }
        }
    }
}

fn get_function<A, R>(
    library: &libloading::Library,
    fn_name: String,
) -> Result<libloading::Symbol<'_, unsafe extern "C" fn(A, ...) -> R>> {
    let func = unsafe { library.get::<unsafe extern "C" fn(A, ...) -> R>(fn_name.as_bytes()) }
        .wrap_err("Failed getting function");

    return func;
}

unsafe fn start_hs(lib: &libloading::Library) -> Result<()> {
    let hs_init: libloading::Symbol<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char)> =
        lib.get(b"hs_init")?;
    hs_init(std::ptr::null_mut(), std::ptr::null_mut()); // Initialize Haskell runtime
    Ok(())
}

unsafe fn end_hs(lib: &libloading::Library) -> Result<()> {
    let hs_exit: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"hs_exit")?;
    hs_exit(); // De-initialize Haskell runtime
    Ok(())
}

fn k_main() -> Result<i32> {
    unsafe {
        let lib = libloading::Library::new("./Foo.dll")?;
        //                                                    argc        argv
        let hs_init: libloading::Symbol<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char)> =
            lib.get(b"hs_init")?;
        let hs_exit: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"hs_exit")?;

        hs_init(std::ptr::null_mut(), std::ptr::null_mut()); // Initialize Haskell runtime
        let result = call_dynamic(&lib, 10, 10).wrap_err("Failed calling function");
        hs_exit(); // De-initialize Haskell runtime

        return result;
    }
}

fn call_dynamic(lib: &libloading::Library, a: i32, b: i32) -> Result<i32> {
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"foobar")?;
        Ok(func(a, b))
    }
}
