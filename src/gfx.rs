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
                resize: false, // TODO allow resize
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

    pub fn sound(&self) {
        // NOT IMPLEMENTED
    }

    pub fn handle_event(&mut self) {
        // NOT IMPLEMENTED
    }
}
