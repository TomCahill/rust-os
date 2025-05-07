#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] // don't mangle the name of this function, since we expect the linker to be looking for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // VGA buffer address

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write the character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // write the colour attribute
        }
    }

    loop {}
}