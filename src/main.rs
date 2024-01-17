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
pub extern "C" fn kernel_main(
    magic: u32,
    _info: *const MultibootInfo,
) -> ! {
    let mut writer = TerminalWriter::new();
    let info: MultibootInfo = unsafe { *_info };

    writer.write(b"Magic: ");
    writer.printhex(magic);
    writer.putchar(b'\n');

    writer.write(b"Mem Lower: ");
    writer.printint(info.mem_lower);
    writer.putchar(b'\n');

    writer.write(b"Mem Upper: ");
    writer.printint(info.mem_upper);
    writer.putchar(b'\n');

    writer.write(b"Boot Loader: ");
    writer.write(unsafe {
        core::slice::from_raw_parts(info.boot_loader_name, 4)
    });

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
pub unsafe extern "C" fn memset(
    dest: *mut u8,
    val: i32,
    n: usize,
) -> *mut u8 {
    let val = val as u8;
    for i in 0..n {
        *dest.add(i) = val;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    dest: *mut u8,
    src: *const u8,
    n: usize,
) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}
