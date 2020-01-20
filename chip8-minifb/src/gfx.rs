use minifb::{Key, KeyRepeat};
use std::time::{Duration, Instant};

pub struct Gfx {
    window: minifb::Window,
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
        let mut window = window.unwrap();
        // we want 60 frames per seconds
        window.limit_update_rate(Some(Duration::from_secs(1).checked_div(60).unwrap()));
        let gfx = Gfx {
            window,
            current_time: Instant::now(),
            width,
            height,
            buffer: vec![0; width * height],
        };

        return Ok(gfx);
    }

    pub fn update(&mut self, arr: &[bool]) {
        for i in 0..arr.len() {
            // TODO UNSAFE
            self.buffer[i] = match arr[i] {
                true => 0xFFe6FFFF,
                false => 0xFF32321e,
            }
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
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
        self.window
            .get_keys_pressed(KeyRepeat::Yes)
            .unwrap()
            .iter()
            .for_each(|keys| match keys {
                Key::Key1 => key[0x1] = true,
                Key::Key2 => key[0x2] = true,
                Key::Key3 => key[0x3] = true,
                Key::Key4 => key[0xC] = true,

                Key::Q => key[0x4] = true,
                Key::W => key[0x5] = true,
                Key::E => key[0x6] = true,
                Key::R => key[0xD] = true,

                Key::A => key[0x7] = true,
                Key::S => key[0x8] = true,
                Key::D => key[0x9] = true,
                Key::F => key[0xE] = true,

                Key::Z => key[0xA] = true,
                Key::X => key[0x0] = true,
                Key::C => key[0xB] = true,
                Key::V => key[0xF] = true,
                _ => (),
            });
    }

    /// reset all keys in the array to false
    pub fn clear_key(key: &mut [bool]) {
        for k in key.iter_mut() {
            *k = false;
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
