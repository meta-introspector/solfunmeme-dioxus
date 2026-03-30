use std::os::raw::c_char;

#[repr(C)]
pub struct ZosPluginVTable {
    pub name: unsafe extern "C" fn() -> *const c_char,
    pub execute: unsafe extern "C" fn(),
}