#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(x86_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use x86_rust_os::{print, println};
use core::panic::PanicInfo;

// #[no_mangle] ensures that the Rust compiler really outputs a function with the name _start.
// Without it the compiler would generate some cryptic name "_ZN3blog_os4_start7hb173feds945531caE"
// to give every function a unique name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting x86 Rust OS!");
    x86_rust_os::init();
    
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
} // Diverging function "-> !" never returns

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_rust_os::test_panic_handler(info)
}