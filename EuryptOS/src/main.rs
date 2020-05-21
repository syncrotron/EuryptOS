//Forbids the use of standard library. Standard library is not architecture
//agnostic and depends on the operating system
#![no_std]
//Turns off rusts normal entry point chain (Looking for main)
#![no_main]

use core::panic::PanicInfo;

//This is called when there is a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static WELCOME: &[u8] = b"Welcome to EuryptOS!";

//Turns off name mangling so rust spits out _start
#[no_mangle]
//Indicates rust should use C calling convention.
//Named _start as it is default in entry point for most OS's
pub extern "C" fn _start() -> ! {

    let vga_buffer = 0xb8000 as *mut u8; // buffer located at 0xb8000

    for (i, &byte) in WELCOME.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x6; //0x6 is the color byte for brown
        }
    }
    loop{}
}
