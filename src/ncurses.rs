use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Gfx {
    window: pancurses::Window,
    frequency: Duration,
    current_time: Instant,
    width: usize,
    height: usize,
}

impl Gfx {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let window = pancurses::initscr();
        let window = window
            .subwin(height as i32 + 2, width as i32 + 2, 0, 0)
            .map_err(|e| e.to_string())?; // no idea of what it is
        Ok(Gfx {
            window,
            // we want 30 frames per seconds
            frequency: Duration::from_secs(1).checked_div(30).unwrap(),
            current_time: Instant::now(),
            width,
            height,
        })
    }

    pub fn update(&mut self, arr: &[u8]) {
        match self.frequency.checked_sub(self.current_time.elapsed()) {
            None => (), // we are slow
            Some(t) => sleep(t),
        }

        self.window.clear();
        self.window.draw_box('|', '-');
        for y in 0..self.height {
            for x in 0..self.width {
                if arr[x + y * self.width] == 0 {
                    self.window.mvaddch(y as i32 + 1, x as i32 + 1, ' ');
                } else {
                    self.window.mvaddch(y as i32 + 1, x as i32 + 1, 'X');
                }
            }
        }
        self.window.refresh();

        self.current_time = Instant::now();
    }

    /// do some sound
    pub fn sound(&self) {
        // TODO better sound
        println!("\x1ba"); // bell?
    }

    /// update the array with the key currently pressed
    pub fn update_key(&mut self, key: &mut [bool]) {}

    /// update all related gfx event (window is closed, resized, whatevered)
    /// if the program need to exit then this function return `true`
    pub fn handle_event(&self) -> bool {
        false
    }
}
