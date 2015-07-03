extern crate libc;

use std::ffi;
use libc::{c_char, c_int, c_uint, uint64_t,  c_double};

// Return codes used by module during (un)initialization
pub const ZBX_MODULE_OK: c_int = 0;
pub const ZBX_MODULE_FAIL: c_int = -1;

// Module API versions
pub const ZBX_MODULE_API_VERSION_ONE: c_int = 1;

// Flags for commands
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
    pub fn new(key: &str,
           flags: u32,
           function: extern "C" fn(*mut ZabbixRequest, *mut ZabbixResult) -> i32,
           test_param: &str) -> ZabbixMetric {
        let c_key = ffi::CString::new(key).unwrap();
        let c_test_param = ffi::CString::new(test_param).unwrap();

        ZabbixMetric {
            key: c_key.as_ptr(),
            flags: flags as c_uint,
            function: function,
            test_param: c_test_param.as_ptr(),
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

    pub fn set_str_result(result: *mut ZabbixResult, value: String) {
        unsafe {
            (*result)._type |= AR_STRING;
            // TODO: set string
        }
    }

    pub fn set_text_result(result: *mut ZabbixResult, value: String) {
        unsafe {
            (*result)._type |= AR_TEXT;
            // TODO: set text
        }
    }

    // TODO: Implement set_log_result(...)

    pub fn set_msg_result(result: *mut ZabbixResult, value: String) {
        unsafe {
            (*result)._type |= AR_MESSAGE;
            // TODO: set message
        }
    }
}

