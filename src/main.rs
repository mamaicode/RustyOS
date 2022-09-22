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
            .write_volatile(0x30);                                     // Add color
    }
    loop{
        hlt();                                                         // Halt instruction for CPU to save electricity 
    }
}

#[allow(unused)]                                                       // Represents a 16 bit color used for vga display.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color16 {
    Black   =   0x0,
    Blue    =   0x1,
    Green   =   0x2,
    Cyan    =   0x3,
    Red     =   0x4,
    Magenta =   0x5,
    Brown   =   0x6,
    Pink    =   0xD,
    Yellow  =   0xE,
    White   =   0xF,
    LightGrey   =   0x7,
    DarkGrey    =   0x8,
    LightBlue   =   0x9,
    LightGreen  =   0xA,
    LightCyan   =   0xB,
    LightRed    =   0xC,
}