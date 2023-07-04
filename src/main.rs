// Disable linking the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/* 
Disable name mangling so the Rust compiler outputs the function as _start()
and not a random name. Otherwise, the OS would not have a start function.
*/
#[no_mangle]
// The operating system entry point, analogous to "fn main()"
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}

// The custom panic handler implementation
// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}