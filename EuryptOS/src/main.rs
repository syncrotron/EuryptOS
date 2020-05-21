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

//Turns off name mangling so rust spits out _start
#[no_mangle]
//Indicates rust should use C calling convention.
//Named _start as it is default in entry point for most OS's
pub extern "C" fn _start() -> ! {
    loop{}
}
