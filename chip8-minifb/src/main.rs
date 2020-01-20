use std::env;
use std::time::{Duration, Instant};

mod gfx;
use crate::gfx::Gfx;

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

    let mut last_instruction_run_time = Instant::now();
    let mut updated = 0;
    loop {
        if gfx.handle_event() {
            break;
        }
        gfx.update_key(&mut chip.key);

        if last_instruction_run_time.elapsed() > Duration::from_millis(5) {
            last_instruction_run_time = Instant::now();
            chip.cycle();

            if let Some(screen) = chip.update() {
                updated += 1;
                gfx.update(screen);
            }
            if updated > 3 {
                updated = 0; // reset the keys every 3 frames
                Gfx::clear_key(&mut chip.key);
            }
            if chip.sound() {
                gfx.sound();
            }
        }
    }
}
