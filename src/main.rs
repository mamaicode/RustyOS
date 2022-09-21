#![no_std]                                                              // Run the program without OS
#![no_main]
#![feature(core_intrinsics)]                                            // Use LLVM intrinsic functions

use core::intrinsics;
use core::panic::PanicInfo;                                             // Allow panic handler to inspect where panic occured

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();                                                // Crash
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer
            .offset(1)                                                 // Increment pointer address by 1 to 0xb8001 
            .write_volatile(0xC0);                                     // Add color
    }

    loop{}
}


