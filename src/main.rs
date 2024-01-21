#![allow(bad_asm_style)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[macro_use]
mod vga;
mod libc;
mod multiboot;

use multiboot::MultibootInfo;
use vga::TerminalWriter;

use core::{
    arch::global_asm,
    panic::PanicInfo
};

//extern crate alloc;

// Include boot.s which defines _start as inline assembly in main. This
// allows us to do more fine grained setup than if we used a naked _start
// function in rust. Theoretically we could use a naked function + some
// inline asm, but this seems much more straight forward.
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main(
    magic: u32,
    info: *const MultibootInfo,
) -> i32 {
    TerminalWriter::init();

    println!("Magic: {:#x}", magic);
    unsafe {
        multiboot::print_mmap_sections(info);
    }

    0
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!");
    if let Some(location) = info.location() {
        println!("{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        println!("At unknown location");
    }
    loop {}
}
