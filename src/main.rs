#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// Ensure that the Rust compiler really outputs a function with the name _start.
// Without the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE
// to give every function a unique name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
} // Diverging function "-> !" never returns