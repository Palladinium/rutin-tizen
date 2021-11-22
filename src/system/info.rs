use std::ffi::{CStr, CString};
use std::ptr;

use libc::{c_char, c_int, c_void};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Invalid parameter
    InvalidParameter,
    /// Out of memory
    OutOfMemory,
    /// An input/output error occurred when reading value from system
    IoError,
    /// No permission to use the API
    PermissionDenied,
    /// Not supported parameter (Since 3.0)
    NotSupported,
    /// Unknown error
    Other(c_int),
}

impl From<c_int> for Error {
    fn from(i: c_int) -> Self {
        match i {
            rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_INVALID_PARAMETER => {
                Error::InvalidParameter
            }
            rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_OUT_OF_MEMORY => {
                Error::OutOfMemory
            }
            rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_IO_ERROR => Error::IoError,
            rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_PERMISSION_DENIED => {
                Error::PermissionDenied
            }
            rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_NOT_SUPPORTED => {
                Error::NotSupported
            }
            _ => Error::Other(i),
        }
    }
}

pub fn get_platform_bool(key: &str) -> Result<bool> {
    let key = CString::new(key).unwrap();
    let mut value = false;

    let ret = unsafe {
        rutin_tizen_sys::system_info_get_platform_bool(key.as_ptr(), &mut value as *mut bool)
    };

    if ret == rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_NONE {
        Ok(value)
    } else {
        Err(Error::from(ret))
    }
}

pub fn get_platform_string(key: &str) -> Result<String> {
    let key = CString::new(key).unwrap();
    let mut value_ptr = ptr::null_mut();

    let ret = unsafe {
        rutin_tizen_sys::system_info_get_platform_string(
            key.as_ptr(),
            &mut value_ptr as *mut *mut c_char,
        )
    };

    if ret == rutin_tizen_sys::system_info_error_e_SYSTEM_INFO_ERROR_NONE {
        unsafe {
            let value = CStr::from_ptr(value_ptr).to_str().unwrap().to_owned();
            libc::free(value_ptr as *mut c_void);
            Ok(value)
        }
    } else {
        Err(Error::from(ret))
    }
}
