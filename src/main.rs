#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(tox::test_runner)] // use our test runner
#![reexport_test_harness_main = "test_main"] // re-export the test harness main function

// mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

// Panic Handling
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n\n!!! PANIC !!!: {}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tox::test_panic_handler(info)
}

// Entry Point
#[unsafe(no_mangle)] // don't mangle the name of this function, since we expect the linker to be looking for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("");
    println!("===========================================");
    println!("|    88888888888 .d88888b. Y88b   d88P    |");
    println!("|        888    d88P   Y88b Y88b d88P     |");
    println!("|        888    888     888  Y88o88P      |");
    println!("|        888    888     888   Y888P       |");
    println!("|        888    888     888   d888b       |");
    println!("|        888    888     888  d88888b      |");
    println!("|        888    Y88b. .d88P d88P Y88b     |");
    println!("|        888      Y88888P  d88P   Y88b    |");
    println!("===========================================");
    println!("");
    println!("Hello World{}", "!");
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);

    #[cfg(test)]
    test_main();

    loop {}
}

// Test Framework
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
