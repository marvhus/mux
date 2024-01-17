#![allow(bad_asm_style)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;
use core::panic::PanicInfo;

mod vga;

// Include boot.s which defines _start as inline assembly in main. This
// allows us to do more fine grained setup than if we used a naked _start
// function in rust. Theoretically we could use a naked function + some
// inline asm, but this seems much more straight forward.
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = vga::TerminalWriter::new();
    writer.write(b"Hello, Kernel!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}
