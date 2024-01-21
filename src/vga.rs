use core::{
    fmt::Write,
    sync::atomic::{
        AtomicU8,
        AtomicUsize,
        Ordering,
    },
    cell::UnsafeCell,
};

// NOTE: This should be safe, TerminalWriter is synbc, and AtomicPtr satisfies
// the constraint on a global. We initialize this at the start of main, so everyone
// else should be able to access the pointer without crashing
pub static TERMINAL_WRITER: SafeCell<TerminalWriter> = SafeCell::new(TerminalWriter::new());

macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write as FmtWrite;

            // Access the SafeCell and get a mutable reference to TerminalWriter
            let writer = $crate::vga::TERMINAL_WRITER.get_mut();

            // Use the fmt::Write trait to write to TerminalWriter
            write!(writer, $($arg)*).expect("Failed to print")
        }
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

pub struct SafeCell<T>(UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SafeCell<T> {}

#[allow(dead_code)]
impl <T> SafeCell<T> {
    pub const fn new(value: T) -> Self {
        SafeCell(UnsafeCell::new(value))
    }

    pub fn get(&self) -> &T {
        // Safety: This is safe because we are providing a shared reference.
        unsafe { &*self.0.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        // Safety: This is safe because we are providing a mutable reference.
        unsafe { &mut *self.0.get() }
    }

    pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // Safety: Ensure that the mutable reference passed to the closure is exclusive.
        unsafe {
            f(&mut *self.0.get())
        }
    }
}

// Hardware Text Mode Colors
#[allow(dead_code)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

const fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

const fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct TerminalWriter {
    pub row: AtomicUsize,
    pub col: AtomicUsize,
    pub color: AtomicU8,
    pub buffer: *mut u16,
}

impl TerminalWriter {
    const fn new() -> TerminalWriter {
        let row = AtomicUsize::new(0);
        let col = AtomicUsize::new(0);
        let color = vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
        let buffer = 0xB8000 as *mut u16;

        TerminalWriter {
            row,
            col,
            color: AtomicU8::new(color),
            buffer,
        }
    }

    pub fn init() -> &'static SafeCell<TerminalWriter> {
        let color = TERMINAL_WRITER.with(|t| t.color.load(Ordering::Relaxed));

        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *TERMINAL_WRITER.get_mut().buffer.add(index) = vga_entry(b' ', color);
                }
            }
        }

        &TERMINAL_WRITER
    }

    #[allow(dead_code)]
    pub fn set_color(&self, color: u8) {
        self.color.store(color, Ordering::Relaxed);
    }

    fn putentryat(&self, c: u8, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        unsafe {
            *self.buffer.add(index) = vga_entry(c, color)
        }
    }

    fn putchar(&self, c: u8) {
        if c == b'\n' {
            self.row.fetch_add(1, Ordering::Relaxed);
            self.col.store(0, Ordering::Relaxed);
            return;
        }

        let row = self.row.load(Ordering::Relaxed);
        let col = self.col.fetch_add(1, Ordering::Relaxed);
        let color = self.color.load(Ordering::Relaxed);
        self.putentryat(c, color, col, row);
        if col >= VGA_WIDTH {
            self.col.store(0, Ordering::Relaxed);
            if row == VGA_HEIGHT {
                self.row.store(0, Ordering::Relaxed);
            }
        }
    }

    fn write(&self, data: &[u8]) {
        for c in data {
            self.putchar(*c);
        }
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

unsafe impl Sync for TerminalWriter {}
