// Disable the standard library
#![no_std]
// Tell the rust compiler we don't want to use the normal entry point chain
#![no_main]

use core::panic::PanicInfo;

// The custom panic handler implementation
// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* 
Disable name mangling so the Rust compiler outputs the function as _start()
and not a random name. Otherwise, the OS would not have a start function.
*/
#[no_mangle]
// The operating system entry point, analogous to "fn main()"
pub extern "C" fn _start() -> ! {
    loop {}
}