#![allow(dead_code)]

use std::{fmt, collections::HashMap, char};
use lazy_static::lazy_static;
use thiserror::Error;

type Bit = bool;

const COLUMNS: usize = 64;
const ROWS: usize = 32;

lazy_static! {
    // TODO - These should be stored in the interpretter section 
    // of memory (addresses 0x000 to 0x1FF)

    /// Sprites represent characters as an 8 wide by 5 high grid of bits.
    pub static ref SPRITES: HashMap<char, [u8; 5]> = {
        let mut sprites = HashMap::new();

        sprites.insert('0', [0xF0, 0x90, 0x90, 0x90, 0xF0]);
        sprites.insert('1', [0x20, 0x60, 0x20, 0x20, 0x70]);
        sprites.insert('2', [0xF0, 0x10, 0xF0, 0x80, 0xF0]);
        sprites.insert('3', [0xF0, 0x10, 0xF0, 0x10, 0xF0]);

        sprites.insert('4', [0x90, 0x90, 0xF0, 0x10, 0x10]);
        sprites.insert('5', [0xF0, 0x80, 0xF0, 0x10, 0xF0]);
        sprites.insert('6', [0xF0, 0x80, 0xF0, 0x90, 0xF0]);
        sprites.insert('7', [0xF0, 0x10, 0x20, 0x40, 0x40]);

        sprites.insert('8', [0xF0, 0x90, 0xF0, 0x90, 0xF0]);
        sprites.insert('9', [0xF0, 0x90, 0xF0, 0x10, 0xF0]);
        sprites.insert('A', [0xF0, 0x90, 0xF0, 0x90, 0x90]);
        sprites.insert('B', [0xE0, 0x90, 0xE0, 0x90, 0xE0]);

        sprites.insert('C', [0xF0, 0x80, 0x80, 0x80, 0xF0]);
        sprites.insert('D', [0xE0, 0x90, 0x90, 0x90, 0xE0]);
        sprites.insert('E', [0xF0, 0x80, 0xF0, 0x80, 0xF0]);
        sprites.insert('F', [0xF0, 0x80, 0xF0, 0x80, 0x80]);

        sprites
    };
}

#[derive(Clone, Copy, Debug)]
pub struct PixelLocation {
    pub x: usize,
    pub y: usize,
}

impl PixelLocation {
    pub fn to_array_index(&self) -> usize {
        let wrapped_y = self.y % ROWS;
        let wrapped_x = self.x % COLUMNS;

        wrapped_x + (wrapped_y * COLUMNS)
    }
}

pub struct Chip8Display {
    pixels: [Bit; COLUMNS * ROWS],
}

impl Chip8Display {
    pub fn new() -> Self {
        Chip8Display {
            pixels: [false as Bit; COLUMNS * ROWS]
        }
    }

    /// Set the provided pixel to value.
    pub fn set_pixel(&mut self, pixel: PixelLocation, value: Bit) -> Result<(), ()> {
        let idx = pixel.to_array_index();
        let pixel = &mut self.pixels[idx];

        *pixel = value;

        Ok(())
    }

    pub fn get_state(&self) -> &[Bit] {
        &self.pixels
    }

    /// Return a mutable reference to the state of the display.
    pub fn get_state_mut(&mut self) -> &mut [Bit] {
        &mut self.pixels
    }

    /// Return the value (on or off) of the pixel at the provided point.
    pub fn pixel_at_location(&self, point: PixelLocation) -> Bit {
        let idx = point.to_array_index();
        self.pixels[idx]
    }

    /// Renders the current state to the output device (terminal UI)
    pub fn render(&self) -> () {
        println!("{}", self);
    }
}

impl fmt::Display for Chip8Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let idx = row * COLUMNS + col;

                let char = if self.pixels[idx] {
                    "■"
                } else {
                    " "
                };

                write!(f, "{} ", char)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}


#[derive(Error, Debug)]
pub enum DisplayError {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pixel_in_bounds_updates_display_state() {
        let mut d = Chip8Display::new();

        let origin = PixelLocation {
            x: 0,
            y: 0,
        };
        
        assert_eq!(d.get_state()[origin.to_array_index()], false); // All pixels are initially "off" (false)
        assert_eq!(d.set_pixel(origin, true), Ok(())); // Updating valid pixel should succeed
        assert_eq!(d.get_state()[origin.to_array_index()], true); // Pixel we set should now be "on" (true)
        
        let middle_of_screen = PixelLocation {
            x: 54,
            y: 28
        };

        assert_eq!(d.get_state()[middle_of_screen.to_array_index()], false); // All pixels are initially "off" (false)
        assert_eq!(d.set_pixel(middle_of_screen, true), Ok(())); // Updating valid pixel should succeed
        assert_eq!(d.get_state()[middle_of_screen.to_array_index()], true); // Pixel we set should now be "on" (true)
    }

    #[test]
    fn set_pixel_out_of_bounds_wraps_to_in_bounds() {
        let mut d = Chip8Display::new();

        let oob_1 = PixelLocation {
            x: 129, // Max is 64
            y: 64   // Max is 32
        };

        assert_eq!(oob_1.to_array_index(), 1); // Array index should wrap around

        assert_eq!(d.get_state()[oob_1.to_array_index()], false); // All pixels are initially "off" (false)
        assert_eq!(d.pixel_at_location(oob_1), false);
        assert_eq!(d.set_pixel(oob_1, true), Ok(())); // Updating oob pixel should succeed
        assert_eq!(d.get_state()[oob_1.to_array_index()], true); // Pixel we set should now be "on" (true)
        assert_eq!(d.pixel_at_location(oob_1), true);
        
        let oob_2 = PixelLocation {
            x: 129,
            y: 66
        };

        assert_eq!(oob_2.to_array_index(), 129); // Array index should wrap around

        assert_eq!(d.get_state()[oob_2.to_array_index()], false); // All pixels are initially "off" (false)
        assert_eq!(d.pixel_at_location(oob_2), false);
        assert_eq!(d.set_pixel(oob_2, true), Ok(())); // Updating oob pixel should succeed
        assert_eq!(d.get_state()[oob_2.to_array_index()], true); // Pixel we set should now be "on" (true)
        assert_eq!(d.pixel_at_location(oob_2), true);
    }
}

