#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use crate::vga_buffer::{Color, ScreenWriter};

mod vga_buffer;

// Ensure that the Rust compiler really outputs a function with the name _start.
// Without the compiler would generate some cryptic _ZN3blog_os4_start7hb173feds945531caE
// to give every function a unique name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    println!("Hello World!");
    println!("My favorite numbers are {} and {}", 3, 13);
    panic!("some panic");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
} // Diverging function "-> !" never returns