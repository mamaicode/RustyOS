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
    let text = b"Mom I am twitch famous";

    let mut cursor = Cursor {
        position: 0,
        foreground: Color16::Red,
        background: Color16::Black,
    };
    cursor.print(text);
    loop{
        hlt();                                                         // Halt instruction for CPU to save electricity 
    }
}

#[allow(unused)]                                                       // Represents a 16 bit color used for vga display.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color16 {                                                     // Enum template for 16 basic colors
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

struct Cursor {                                                         // Cursor struct that handles raw memory manipulation and converts Color16 to readable VGA                                              
    position: isize,
    foreground: Color16,
    background: Color16,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {                                  // Input uses a raw byte stream 
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}
