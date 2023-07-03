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
