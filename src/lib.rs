extern crate libc;

use std::{ffi, ptr};
use libc::{c_char, c_int, c_uint, uint64_t,  c_double, malloc, strncpy};

// Return codes used by module during (un)initialization
pub const ZBX_MODULE_OK: c_int = 0;
pub const ZBX_MODULE_FAIL: c_int = -1;

// Module API versions
pub const ZBX_MODULE_API_VERSION_ONE: c_int = 1;

// Flags for commands
// Item does not accept parameters
pub const CF_NOPARAMS: c_uint = 0;

// Item accepts either optional or mandatory parameters
pub const CF_HAVEPARAMS: c_uint = 1;

// Item is defined in a loadable module
pub const CF_MODULE: c_uint = 2;

// Item is defined as user parameter
pub const CF_USERPARAMETER: c_uint = 4;

// Agent result types
pub const AR_UINT64: c_int = 1;
pub const AR_DOUBLE: c_int = 2;
pub const AR_STRING: c_int = 4;
pub const AR_TEXT: c_int = 8;
pub const AR_LOG: c_int = 16;
pub const AR_MESSAGE: c_int = 32;

// Return codes used by item callbacks
pub const SYSINFO_RET_OK: c_int = 0;
pub const SYSINFO_RET_FAIL: c_int = 1;

#[repr(C)]
#[derive(Copy)]
pub struct ZabbixMetric {
    key: *const c_char,
    flags: c_uint,
    function: extern "C" fn(*mut ZabbixRequest, *mut ZabbixResult) -> c_int,
    test_param: *const c_char,
}

impl Clone for ZabbixMetric {
    fn clone(&self) -> Self { *self }
}

impl ZabbixMetric {
    pub fn new(key: Option<&str>,
               flags: Option<u32>,
               function: Option<extern "C" fn(*mut ZabbixRequest, *mut ZabbixResult) -> i32>,
               test_param: Option<&str>) -> ZabbixMetric {

        let c_key: *const c_char = match key {
            Some(ref k) => ffi::CString::new(*k).unwrap().as_ptr(),
            None        => ptr::null(),
        };

        let flags: c_uint = match flags {
            Some(f) => f,
            None    => CF_NOPARAMS,
        };

        let function: extern "C" fn(*mut ZabbixRequest, *mut ZabbixResult) -> c_int = match function {
            Some(callback) => callback,
            None           => dummy_callback,
        };

        let c_test_param: *const c_char = match test_param {
            Some(ref t) => ffi::CString::new(*t).unwrap().as_ptr(),
            None        => ptr::null(),
        };

        ZabbixMetric {
            key: c_key,
            flags: flags,
            function: function,
            test_param: c_test_param,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ZabbixRequest {
    key: *const c_char,
    nparam: c_int,
    params: *const *const c_char,
    lastlogsize: uint64_t,
    mtime: c_int,
}

impl Clone for ZabbixRequest {
    fn clone(&self) -> Self { *self }
}

impl ZabbixRequest {
    pub fn get_params<'a>(request: *mut ZabbixRequest) -> Vec<&'a[u8]> {
        unsafe {
            let len = (*request).nparam;
            let mut v = Vec::new();

            for i in 0..len {
                let ptr = (*request).params.offset(i as isize);
                let param = ffi::CStr::from_ptr(*ptr).to_bytes();
                v.push(param);
            }

            v
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct zbx_log_t {
    value: *const c_char,
    source: *const c_char,
    lastlogsize: uint64_t,
    timestamp: c_int,
    severity: c_int,
    logeventid: c_int,
    mtime: c_int,
}

impl Clone for zbx_log_t {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub struct ZabbixResult {
    _type: c_int,
    ui64: uint64_t,
    dbl: c_double,
    _str: *const c_char,
    text: *const c_char,
    msg: *const c_char,
    logs: *const *const zbx_log_t,
}

impl Clone for ZabbixResult {
    fn clone(&self) -> Self { *self }
}

impl ZabbixResult {
    pub fn set_uint64_result(result: *mut ZabbixResult, value: u64) {
        unsafe {
            (*result)._type |= AR_UINT64;
            (*result).ui64 = value as uint64_t;
        }
    }

    pub fn set_f64_result(result: *mut ZabbixResult, value: f64) {
        unsafe {
            (*result)._type |= AR_DOUBLE;
            (*result).dbl = value as c_double;
        }
    }

    pub fn set_str_result(result: *mut ZabbixResult, value: &str) {
        unsafe {
            (*result)._type |= AR_STRING;
            (*result)._str = string_to_malloc_ptr(value);
        }
    }

    pub fn set_text_result(result: *mut ZabbixResult, value: &str) {
        unsafe {
            (*result)._type |= AR_TEXT;
            (*result).text = string_to_malloc_ptr(value);
        }
    }

    // TODO: Implement set_log_result(...)

    pub fn set_msg_result(result: *mut ZabbixResult, value: &str) {
        unsafe {
            (*result)._type |= AR_MESSAGE;
            (*result).msg = string_to_malloc_ptr(value);
        }
    }
}

// Dummy Zabbix item callback function.
// Callback is used by NULL-key Zabbix items to specify the
// end of the items list.
// Do not use this callback directly in your crates.
#[no_mangle]
#[allow(unused_variables)]
pub extern fn dummy_callback(request: *mut ZabbixRequest, result: *mut ZabbixResult) -> c_int {
    SYSINFO_RET_OK
}

// When the result of a Zabbix item is text (string, text and message)
// Zabbix expects to receive a pre-allocated pointer with the result
// string, which is free(3)'d by Zabbix once done with the result.
unsafe fn string_to_malloc_ptr(src: &str) -> *mut c_char {
    let c_src = ffi::CString::new(src).unwrap();
    let len = c_src.to_bytes_with_nul().len() as u64;

    let dst = malloc(len) as *mut c_char;
    strncpy(dst, c_src.as_ptr(), len);

    dst
}

