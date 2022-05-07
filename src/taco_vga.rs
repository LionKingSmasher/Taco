#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum VGAColorCode {
    BLACK     = 0,
    BLUE      = 1,
    GREEN     = 2,
    CYAN      = 3,
    RED       = 4,
    MAGENTA   = 5,
    BROWN     = 6,
    L_GRAY    = 7,
    D_GRAY    = 8,
    L_BLUE    = 9,
    L_GREEN   = 10,
    L_CYAN    = 11,
    L_RED     = 12,
    L_MAGENTA = 13,
    YELLO     = 14,
    WHITE     = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TacoColor(u8);

impl TacoColor {
    fn new(fore : VGAColorCode, back : VGAColorCode) -> TacoColor {
        TacoColor((back as u8) << 4 | (fore as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct TacoScreenChar {
    c : u8,
    color_code : TacoColor,
}

const TACO_VGA_BUFFER_HEIGHT : usize = 25;
const TACO_VGA_BUFFER_WEIGHT : usize = 80;

#[repr(transparent)]
pub struct TacoScreenBuffer {
    chars : [[TacoScreenChar; TACO_VGA_BUFFER_WEIGHT]; TACO_VGA_BUFFER_HEIGHT],
}

pub struct TacoVgaWriter {
    row_pos    : usize,
    col_pos    : usize,
    color_code : TacoColor,
    buffer     : &'static mut TacoScreenBuffer
}

impl TacoVgaWriter {
    pub fn new(row_pos : usize, col_pos : usize, color_code : TacoColor, buf : &'static mut TacoScreenBuffer) -> TacoVgaWriter {
        TacoVgaWriter {
            row_pos,
            col_pos,
            color_code,
            buffer: buf,
        }
    }

    pub fn write_byte(&mut self, byte : u8) {
        let row = self.row_pos;
        let col = self.col_pos;
        let color_code = self.color_code;
        self.buffer.chars[row][col] = TacoScreenChar {
            c : byte,
            color_code
        };
    }
}
