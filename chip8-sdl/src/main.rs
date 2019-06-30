use std::env;

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

    loop {
        gfx.update_screen(&mut chip);
        if gfx.handle_event(&mut chip.key) {
            break;
        }

        chip.cycle();
    }
}
