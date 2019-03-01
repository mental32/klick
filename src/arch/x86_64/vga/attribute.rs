#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Attribute(u8);

impl Attribute {
    pub fn new(foreground: Color, background: Color) -> Self {
        Attribute((background as u8) << 4 | (foreground as u8))
    }

    pub fn default() -> Self {
    	Attribute((Color::Black as u8) << 4 | (Color::White as u8))
    }

    pub fn same(color: Color) -> Self {
        Self::new(color, color)
    }
}
