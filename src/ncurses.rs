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
        pancurses::noecho();
        pancurses::raw();
        let window = pancurses::initscr();
        window.nodelay(true);
        let window = window
            .subwin(height as i32 + 2, width as i32 + 2, 0, 0)
            .map_err(|e| e.to_string())?; // no idea of what it is
        window.nodelay(true);
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
            None => 42, // we are slow
            Some(t) => pancurses::napms(t.as_millis() as i32),
        };

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
        pancurses::beep();
    }

    /// update the array with the key currently pressed
    pub fn update_key(&mut self, key: &mut [bool]) {
        for k in key.iter_mut() {
            *k = false;
        }

        match self.window.getch() {
            Some(pancurses::Input::Character('&')) => key[0] = true,
            Some(pancurses::Input::Character('é')) => key[1] = true,
            Some(pancurses::Input::Character('"')) => key[2] = true,
            Some(pancurses::Input::Character('\'')) => key[3] = true,
            Some(pancurses::Input::Character('a')) => key[4] = true,
            Some(pancurses::Input::Character('z')) => key[5] = true,
            Some(pancurses::Input::Character('e')) => key[6] = true,
            Some(pancurses::Input::Character('r')) => key[7] = true,
            Some(pancurses::Input::Character('q')) => key[8] = true,
            Some(pancurses::Input::Character('s')) => key[9] = true,
            Some(pancurses::Input::Character('d')) => key[10] = true,
            Some(pancurses::Input::Character('f')) => key[11] = true,
            Some(pancurses::Input::Character('w')) => key[12] = true,
            Some(pancurses::Input::Character('x')) => key[13] = true,
            Some(pancurses::Input::Character('c')) => key[14] = true,
            Some(pancurses::Input::Character('v')) => key[15] = true,
            _ => (),
        }
    }

    /// update all related gfx event (window is closed, resized, whatevered)
    /// if the program need to exit then this function return `true`
    pub fn handle_event(&self) -> bool {
        false
    }
}
