use minifb::{Key, KeyRepeat};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Gfx {
    window: minifb::Window,
    frequency: Duration,
    current_time: Instant,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Gfx {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let window = minifb::Window::new(
            "chip-8 Emulator",
            width,
            height,
            minifb::WindowOptions {
                resize: true, // TODO allow resize
                scale: minifb::Scale::X8,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };
        let gfx = Gfx {
            window: window.unwrap(),
            // we want 60 frames per seconds
            frequency: Duration::from_secs(1).checked_div(60).unwrap(),
            current_time: Instant::now(),
            width,
            height,
            buffer: vec![0; width * height],
        };

        return Ok(gfx);
    }

    pub fn update(&mut self, arr: &[u8]) {
        for i in 0..arr.len() {
            // TODO UNSAFE
            let color = arr[i] as u32;
            self.buffer[i] = (color << 24) | (color << 16) | (color << 8) | color;
        }

        match self.frequency.checked_sub(self.current_time.elapsed()) {
            None => println!("We are SLOW!"),
            Some(t) => sleep(t),
        }
        self.window
            .update_with_buffer(&self.buffer)
            .unwrap_or_else(|e| println!("Window update failed: {}", e));

        self.current_time = Instant::now();
    }

    /// do some sound
    pub fn sound(&self) {
        // TODO better sound
        println!("\x1ba"); // bell?
    }

    /// update the array with the key currently pressed
    pub fn update_key(&mut self, key: &mut [bool]) {
        // clear all keys
        for k in key.iter_mut() {
            *k = false;
        }
        self.window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                // this is broken TODO
                match t {
                    Key::Key1 => key[0] = true,
                    Key::Key2 => key[1] = true,
                    Key::Key3 => key[2] = true,
                    Key::Key4 => key[3] = true,
                    Key::A => key[4] = true,
                    Key::Z => key[5] = true,
                    Key::E => key[6] = true,
                    Key::R => key[7] = true,
                    Key::Q => key[8] = true,
                    Key::S => key[9] = true,
                    Key::D => key[10] = true,
                    Key::F => key[11] = true,
                    Key::W => key[12] = true,
                    Key::X => key[13] = true,
                    Key::C => key[14] = true,
                    Key::V => key[15] = true,
                    _ => (),
                }
            }
        });
        if key.iter().any(|e| *e) {
            println!("FOUND A KEY");
        }
    }

    /// update all related gfx event (window is closed, resized, whatevered)
    /// if the program need to exit then this function return `true`
    pub fn handle_event(&self) -> bool {
        if !self.window.is_open() {
            return true;
        } else if self.window.is_key_down(Key::Escape) {
            return true;
        }
        false
    }
}
