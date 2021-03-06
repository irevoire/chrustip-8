use std::env;

mod ncurses;
use crate::ncurses::Gfx;

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Need a game as argument");
            return;
        }
    };

    let mut gfx = Gfx::new(64, 32).unwrap();
    let mut chip = chip8_cpu::new();

    if let Err(e) = chip.load_game(&filename) {
        println!("Can't load game : {}", e);
        return;
    }

    loop {
        if gfx.handle_event() {
            break;
        }
        gfx.update_key(&mut chip.key);
        chip.cycle();

        if let Some(screen) = chip.update() {
            gfx.update(screen);
        }
        if chip.sound() {
            gfx.sound();
        }
    }
}
