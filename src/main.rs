#![allow(bad_asm_style)]
#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;
use core::panic::PanicInfo;

// Include boot.s which defines _start as inline assembly in main. This
// allows us to do more fine grained setup than if we used a naked _start
// function in rust. Theoretically we could use a naked function + some
// inline asm, but this seems much more straight forward.
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
