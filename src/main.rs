use std::{
    ops::ControlFlow,
    time::Duration,
};
use crossterm::event::{self, Event, KeyCode};

use emu_display::{SPRITES, PixelLocation};
use ratatui::Frame;
use tui::*;

mod cpu;
mod emu_display;
mod tui;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut emulator = cpu::Chip8Emu::new();
    let mut tui = make_terminal().expect("Could not create terminal UI.");
    let mut tui_open = true;

    let loc = PixelLocation {
        x: 12,
        y: 12
    };

    emulator.get_display_mut().set_pixel(loc, true).expect("If this errored you are screwed");


    while tui_open {
        // Advance emulator state

        tui.draw(|frame: &mut Frame| {
            tui::draw_frame(frame, emulator.get_display());
        })?;

        if handle_events()?.is_break() {
            tui_open = false;
        }
    }

    Ok(())
}

fn handle_events() -> Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

