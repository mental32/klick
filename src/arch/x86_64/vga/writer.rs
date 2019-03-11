use core::fmt;
use core::default;
use volatile::Volatile;

use super::{Attribute, Color};

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Character {
    pub data: u8,
    pub attr: Attribute,
}

impl Character {
    pub fn new(data: u8, attr: Attribute) -> Self {
        Self { data, attr }
    }

    pub fn as_whitespace(foreground: Color, background: Color) -> Self {
        Self {
            data: b' ',
            attr: Attribute::new(foreground, background)
        }
    }

    pub fn as_default_whitespace() -> Self {
        Self {
            data: b' ',
            attr: Attribute::default()
        }
    }
}

#[repr(transparent)]
struct Buffer {
    data: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub col: usize,
    pub row: usize,
    pub attr: Attribute,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

impl default::Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            attr: Attribute::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
        }
    }

    pub fn write(&mut self, byte: u8) {
        match byte {
            b'\n' => {

                if self.row == BUFFER_HEIGHT - 1 {
                    self.scroll();
                    self.clear_row(BUFFER_HEIGHT - 1);
                    self.row = BUFFER_HEIGHT - 1;
                } else {
                    self.row += 1;
                }

                self.col = 0;
            },

            byte => {
                if self.col == BUFFER_WIDTH {
                    if self.row == BUFFER_HEIGHT - 1 {
                        self.scroll();
                        self.clear_row(BUFFER_HEIGHT - 1);
                        self.row = BUFFER_HEIGHT - 1;
                    } else {
                        self.row += 1;
                    }

                    self.col = 0;
                }

                self.buffer.data[self.row][self.col].write(Character {
                    data: byte,
                    attr: self.attr,
                });

                self.col += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write(byte),
                _ => self.write(0xfe),
            }

        }
    }

    pub fn fill(&mut self, character: Character) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.data[row][col].write(character)
            }
        }
    }

    fn scroll(&mut self) {for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.data[row][col].read();
                self.buffer.data[row - 1][col].write(character);
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Character {
            data: b' ',
            attr: self.attr,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.data[row][col].write(blank);
        }
    }
}
