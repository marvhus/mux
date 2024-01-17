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

    #[allow(dead_code)]
    pub fn printint(&mut self, val: u32) {
        let mut num_digits: u32 = 1;
        loop {
            if val / 10_u32.pow(num_digits) == 0 { break; }
            num_digits += 1;
        }
        for digit in (1..=num_digits).rev() {
            let char = (val / 10_u32.pow(digit - 1)) % 10;
            self.putchar(char as u8 + b'0');
        }
    }

    #[allow(dead_code)]
    pub fn printhex(&mut self, val: u32) {
        const HEX_CHARS: [u8;16] = [
            b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
            b'A', b'B', b'C', b'D', b'E', b'F'
        ];
        for bytes in (0..4).rev() {
            let byte: u8 = ((val >> (bytes * 8)) & 0xFF) as u8;
            let lower: u8 = byte & 0xF;
            let upper: u8 = byte >> 4;
            self.putchar(HEX_CHARS[upper as usize]);
            self.putchar(HEX_CHARS[lower as usize]);
        }
    }

    pub fn putchar(&mut self, c: u8) {
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

    #[allow(dead_code)]
    pub fn newline(&mut self) {
        self.col = 0;
        self.row += 1;
        if self.row == VGA_HEIGHT {
            self.row = 0;
        }
    }
}
