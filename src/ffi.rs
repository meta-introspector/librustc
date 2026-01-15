use crate::loader::RustcHandle;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn librustc_load(path: *const c_char) -> *mut RustcHandle {
    if path.is_null() {
        return std::ptr::null_mut();
    }

    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    match RustcHandle::load(path_str) {
        Ok(handle) => Box::into_raw(Box::new(handle)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn librustc_free(handle: *mut RustcHandle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}
