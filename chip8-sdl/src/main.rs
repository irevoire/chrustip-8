use std::env;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod gfx;

pub fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Need a game as argument");
            return;
        }
    };

    let mut chip = chip8_cpu::new();

    if let Err(e) = chip.load_game(&filename) {
        println!("Can't load game : {}", e);
        return;
    }

    let mut gfx = gfx::init_sdl(960, 480);
    let frequency = Duration::from_secs(1).checked_div(60).unwrap();

    loop {
        let current_time = Instant::now();

        gfx.update_screen(&mut chip);
        if gfx.handle_event(&mut chip.key) {
            break;
        }

        chip.cycle();

        match frequency.checked_sub(current_time.elapsed()) {
            None => println!("We are SLOW!"),
            Some(t) => sleep(t),
        }
    }
}
