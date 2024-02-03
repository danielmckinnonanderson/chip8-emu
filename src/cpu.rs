#[allow(dead_code)]

use crate::display::Chip8Display;

/// All instructions & descriptions are sourced from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#8xy6

pub struct Chip8Emu {
    memory: [u8; 4096],

    /// Registers are referred to as V(hex number).
    /// So register V0 would be accessed as index 0 of this array,
    /// and register VF would be accessed as index 15 of this array.
    v_registers: [u8; 16],

    /// Store addresses that interpretter returns to when finished with a subroutine.
    stack: [u16; 16], 

    display_register: Chip8Display,

    addr_pointer: u16,
    program_counter: u16, /// Stores the currently executing address.
    stack_pointer: u8,    /// Pointer to top-most level of the stack.
    delay_register: u8,
    sound_register: u8,

    paused: bool,
}

impl Chip8Emu {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

