#![allow(dead_code)]

use thiserror::Error;

type Bit = bool;

const COLUMNS: usize = 64;
const ROWS: usize = 32;


pub struct PixelLocation {
    x: usize,
    y: usize,
}

impl PixelLocation {
    pub fn to_array_index(&self) -> usize {
        self.x + (self.y * COLUMNS)
    }
}

pub struct Display {
    state: [Bit; COLUMNS * ROWS],
}


impl Display {
    pub fn new() -> Self {
        Display {
            state: [false as Bit; COLUMNS * ROWS]
        }
    }

    /// Set the provided pixel to value.
    pub fn set_pixel(&mut self, pixel: PixelLocation,value: Bit) -> Result<(), ()> {
        let idx = pixel.to_array_index();
        let pixel = &mut self.state[idx];

        *pixel = value;

        Ok(())
    }

    pub fn get_state(&self) -> &[Bit] {
        &self.state
    }

    /// Return a mutable reference to the state of the display.
    pub fn get_state_mut(&mut self) -> &mut [Bit] {
        &mut self.state
    }

    /// Return the value (on or off) of the pixel at the provided point.
    pub fn pixel_at(&self, point: PixelLocation) -> Bit {
        // TODO - Bounds-check the point's x & y
        let idx = point.to_array_index();
        self.state[idx]
    }
}


#[derive(Error, Debug)]
pub enum DisplayError {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pixel_in_bounds_updates_display_state() {
        let mut d = Display::new();

        let origin = PixelLocation {
            x: 0,
            y: 0,
        };
        
        assert_eq!(d.get_state()[0], false); // All pixels are initially "off" (false)
        assert_eq!(d.set_pixel(origin, true), Ok(())); // Updating valid pixel should succeed
        assert_eq!(d.get_state()[0], true); // Pixel we set should now be "on" (true)
        
        let middle_of_screen = PixelLocation {
            x: 54,
            y: 28
        };

        assert_eq!(d.get_state()[0], false); // All pixels are initially "off" (false)
        assert_eq!(d.set_pixel(middle_of_screen, true), Ok(())); // Updating valid pixel should succeed
        assert_eq!(d.get_state()[0], true); // Pixel we set should now be "on" (true)
    }
}

