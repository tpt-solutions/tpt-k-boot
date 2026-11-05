// SPDX-License-Identifier: MIT OR Apache-2.0
//! tpt-k-boot: UEFI application entry point (scaffold).

#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "efiapi" fn efi_main(_image: *mut c_void, _system_table: *mut c_void) -> usize {
    0 // EFI_SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
