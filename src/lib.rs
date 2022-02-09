use std::{
    ffi::{c_void, CStr, CString},
    ptr::{addr_of_mut, null_mut},
};

use error::InjectorError;
use injector_sys::injector_t;

pub mod error;

pub fn inject_library(pid: u32, path_to_lib: &str) -> Result<(), InjectorError> {
    let mut injector: *mut injector_t = null_mut();
    let mut slib_handle: *mut c_void = null_mut();
    let inj_ptr = addr_of_mut!(injector);
    let res = unsafe { injector_sys::injector_attach(inj_ptr, pid) };
    if res != 0 {
        return Err(InjectorError::AttatchementError(pid, unsafe {
            CStr::from_ptr(injector_sys::injector_error())
                .to_str()
                .unwrap()
        }));
    }
    let path = CString::new(path_to_lib).unwrap();
    let injection_res =
        unsafe { injector_sys::injector_inject(injector, path.as_ptr(), addr_of_mut!(slib_handle)) };
    if injection_res != 0 {
        return Err(InjectorError::InjectionError(
            pid,
            path_to_lib.to_owned(),
            unsafe {
                CStr::from_ptr(injector_sys::injector_error())
                    .to_str()
                    .unwrap()
            },
        ));
    }
    Ok(())
}

#[test]
pub fn test_injection(){
   let lib = "testDLL.dll";
   let result = inject_library(21576, lib).unwrap();
}