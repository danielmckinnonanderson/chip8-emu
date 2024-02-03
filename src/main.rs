use display::*;

mod display;

fn main() {
    let mut display = Chip8Display::new();

    display.set_pixel(PixelLocation { x: 0, y: 0 }, true).expect("How tf did this cause an error");
    display.set_pixel(PixelLocation { x: 12, y: 12}, true).expect("How tf did this cause an error");

    println!("{}", display);
}

