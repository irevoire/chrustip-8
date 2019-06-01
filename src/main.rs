use std::env;

mod gfx;
mod vm;

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Need a game as argument");
            return;
        },
    };

    let mut chip = vm::Chip8::new();
    if let Err(e) = chip.load_game(&filename) {
        println!("Can't load game : {}", e);
        return
    }

    loop {
        chip.cycle();
        chip.update();
    }
}
