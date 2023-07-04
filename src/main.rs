// Disable linking the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/* 
Disable name mangling so the Rust compiler outputs the function as _start()
and not a random name. Otherwise, the OS would not have a start function.
*/
#[no_mangle]
// The operating system entry point, analogous to "fn main()"
pub extern "C" fn _start() -> ! {
    println!("{} World{}", "Hello", "!");
    panic!("Some panic message");

    loop {}
}

// The custom panic handler implementation
// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}