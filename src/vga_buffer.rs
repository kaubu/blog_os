// Representing all possible colours in the VGA Buffer
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
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
    Pink = 13,
    Yellow = 14,
    White = 15,
}
 
/* 
A newtype idiom for a full colour code that represents foreground and
background colour
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

// Represents a single screen character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Guarentees that the struct's fields are laid out exactly like a C struct
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

// Represents the screen size
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Represents the buffer itself
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// The public Writer API, used to write to the screen
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let colour_code = self.colour_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    colour_code,
                };

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        // Todo
    }
}

// Temporary test function
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        colour_code: ColourCode::new(
            Colour::Yellow,
            Colour::Black
        ),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}