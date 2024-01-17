#![allow(bad_asm_style)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;
use core::panic::PanicInfo;

//extern crate alloc;

// Include boot.s which defines _start as inline assembly in main. This
// allows us to do more fine grained setup than if we used a naked _start
// function in rust. Theoretically we could use a naked function + some
// inline asm, but this seems much more straight forward.
global_asm!(include_str!("boot.s"));

mod vga;
use vga::TerminalWriter;

mod multiboot;
use multiboot::MultibootInfo;

#[no_mangle]
pub unsafe extern "C" fn kernel_main(
    multiboot_magic: u32,
    multiboot_header: *const MultibootInfo,
) -> ! {
    let mut writer = TerminalWriter::new();

    writer.write(b"Magic: ");
    writer.printhex(multiboot_magic);
    writer.newline();

    writer.write(b"Mem Lower: ");
    writer.printint((*multiboot_header).mem_lower);
    writer.newline();

    writer.write(b"Mem Upper: ");
    writer.printint((*multiboot_header).mem_upper);
    writer.newline();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = TerminalWriter::new();
    writer.write(b"PANIC!");
    writer.newline();
    if let Some(location) = info.location() {
        writer.write(location.file().as_bytes());
        writer.write(b":");
        writer.printhex(location.line());
        writer.write(b":");
        writer.printhex(location.column());
    } else {
        writer.write(b"At unknown location")
    }
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
