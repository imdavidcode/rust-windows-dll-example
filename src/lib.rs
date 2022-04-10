use windows_sys::{
    Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

use std::ffi::c_void;
use std::ptr;
use windows_sys::Win32::Foundation::HINSTANCE;
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;


#[no_mangle]
pub unsafe extern "system" fn runner(_: *mut c_void) -> u32 {
    println!("Hello world");
    MessageBoxA(0, b"Rust\0".as_ptr(), b"Dll injected\0".as_ptr(), MB_OK);

    1
}


#[no_mangle]
pub extern "system" fn DllMain(dll: HINSTANCE, reason: u32, _: usize) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(dll);
            CreateThread(
                ptr::null_mut(),
                0,
                Some(runner),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
            );
        }
    }

    true
}
