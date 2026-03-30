use std::os::raw::c_char;
use std::ffi::CString;
use zos::plugins::plugin::ZosPluginVTable;

#[no_mangle]
pub extern "C" fn hello_plugin_name() -> *const c_char {
    CString::new("hello").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn hello_plugin_execute() {
    println!("Hello from a zos plugin!");
}

#[no_mangle]
pub extern "C" fn _zos_plugin_create() -> *const ZosPluginVTable {
    static VTABLE: ZosPluginVTable = ZosPluginVTable {
        name: hello_plugin_name,
        execute: hello_plugin_execute,
    };
    &VTABLE
}