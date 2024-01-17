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

fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct TerminalWriter {
    row: usize,
    col: usize,
    color: u8,
    buffer: *mut u16,
}

impl TerminalWriter {
    pub fn new() -> TerminalWriter {
        let color = vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
        let buffer = 0xB8000 as *mut u16;
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *buffer.add(index) = vga_entry(b' ', color);
                }
            }
        }

        TerminalWriter {
            row: 0,
            col: 0,
            color,
            buffer,
        }
    }

    #[allow(dead_code)]
    fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    fn putentryat(&mut self, c: u8, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        unsafe {
            *self.buffer.add(index) = vga_entry(c, color);
        }
    }

    fn putchar(&mut self, c: u8) {
        self.putentryat(c, self.color, self.col, self.row);
        self.col += 1;
        if self.col == VGA_WIDTH {
            self.col = 0;
            self.row += 1;
            if self.row == VGA_HEIGHT {
                self.row = 0;
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        for c in data {
            self.putchar(*c);
        }
    }
}
