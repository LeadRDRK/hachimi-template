use std::os::raw::{c_ulong, c_void};

use windows::{
    core::BOOL,
    Win32::Foundation::{HMODULE, TRUE}
};

use crate::core::Hachimi;

use super::hook;

const DLL_PROCESS_ATTACH: c_ulong = 1;
const DLL_PROCESS_DETACH: c_ulong = 0;

pub static mut DLL_HMODULE: HMODULE = HMODULE(0 as _);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DllMain(hmodule: HMODULE, call_reason: c_ulong, _reserved: *mut c_void) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe { DLL_HMODULE = hmodule; }
        if !Hachimi::init() {
            return TRUE;
        }

        hook::init();
        info!("Attach completed");
    }
    else if call_reason == DLL_PROCESS_DETACH && Hachimi::is_initialized() {
        info!("Unhooking everything");
        Hachimi::instance().interceptor.unhook_all();
    }
    TRUE
}