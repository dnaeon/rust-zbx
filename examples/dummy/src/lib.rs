// Example Zabbix loadable module written in Rust

extern crate zbx;
extern crate rand;

use std::str;
use rand::Rng;

#[no_mangle]
pub extern fn zbx_module_api_version() -> i32 {
    zbx::ZBX_MODULE_API_VERSION_ONE
}

#[no_mangle]
pub extern fn zbx_module_init() -> i32 {
    zbx::ZBX_MODULE_OK
}

#[no_mangle]
pub extern fn zbx_module_uninit() -> i32 {
    zbx::ZBX_MODULE_OK
}

#[no_mangle]
pub extern fn zbx_module_item_list() -> *const zbx::ZBX_METRIC {
    let metrics = vec![
        zbx::Metric::new("rust.echo", zbx::CF_HAVEPARAMS, rust_echo, ""),
        zbx::Metric::new("rust.random", zbx::CF_NOPARAMS, rust_random, ""),
    ];

    zbx::create_items(&metrics)
}

#[no_mangle]
pub extern fn rust_echo(request: *mut zbx::AGENT_REQUEST, result: *mut zbx::AGENT_RESULT) -> i32 {
    let params = zbx::AGENT_REQUEST::get_params(request);

    if params.len() != 1 {
        zbx::AGENT_RESULT::set_msg_result(result, "Invalid number of parameters");
        return zbx::SYSINFO_RET_FAIL;
    }

    let param = match str::from_utf8(params[0]) {
        Ok(p)  => p,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    zbx::AGENT_RESULT::set_str_result(result, param);

    zbx::SYSINFO_RET_OK
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn rust_random(request: *mut zbx::AGENT_REQUEST, result: *mut zbx::AGENT_RESULT) -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u64>();

    zbx::AGENT_RESULT::set_uint64_result(result, num);

    zbx::SYSINFO_RET_OK
}
