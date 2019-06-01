use minifb::{Key, KeyRepeat};

pub struct Gfx {
    window: minifb::Window,
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
                scale: minifb::Scale::X4,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };
        let gfx = Gfx {
            window: window.unwrap(),
            width,
            height,
            buffer: vec![0; width * height],
        };

        return Ok(gfx);
    }

    pub fn destroy(&self) {
        // empty
    }

    pub fn clear(&mut self) {}

    pub fn update(&mut self, arr: &[u8]) {
        for i in 0..arr.len() {
            // TODO UNSAFE
            let color = arr[i] as u32;
            if color != 0 {
                println!("There is a color");
            }

            self.buffer[i] = (color << 24) | (color << 16) | (color << 8) | color;
        }

        self.window
            .update_with_buffer(&self.buffer)
            .unwrap_or_else(|e| println!("Window update failed: {}", e));
    }

    pub fn sound(&self) {
        // NOT IMPLEMENTED
    }

    pub fn handle_event(&mut self) {
        // NOT IMPLEMENTED
    }
}
