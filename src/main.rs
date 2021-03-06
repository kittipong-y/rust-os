#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

extern crate rlibc;

use core::panic::PanicInfo;

mod vga_buffer;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function named "_start" by default.
    vga_buffer::print_something();

    loop { }
}
