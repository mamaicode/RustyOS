#![no_std]                                                              // Run the program without OS
#![no_main]
#![feature(core_intrinsics)]                                            // Use LLVM intrinsic functions
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;                                             // Allow panic handler to inspect where panic occured
use x86_64::instructions::{hlt};

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    }                                                                   // Crash
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    unsafe {
        framebuffer
            .offset(1)                                                 // Increment pointer address by 1 to 0xb8001 
            .write_volatile(0xC0);                                     // Add color
    }
    loop{
        hlt();                                                         // Halt instruction for CPU to save electricity 
    }
}


