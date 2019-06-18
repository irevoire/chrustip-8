use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{rect::Point, video, EventPump, Sdl};

pub struct Gfx {
    pub context: Sdl,
    pub canvas: Canvas<video::Window>,
    event_pump: EventPump,
}

pub fn init_sdl(width: u32, height: u32) -> Gfx {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem
        .window("chip 8", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_scale(width as f32 / 64.0, height as f32 / 32.0)
        .unwrap();
    canvas.set_draw_color(Color::RGB(50, 50, 30));
    canvas.clear();
    canvas.present();

    let event_pump = context.event_pump().unwrap();

    Gfx {
        context,
        canvas,
        event_pump,
    }
}

impl Gfx {
    /// update the screen with the data in the chip8
    pub fn update_screen(&mut self, chip: &mut chip8_cpu::cpu::Cpu) {
        if let Some(screen) = chip.update() {
            self.canvas.set_draw_color(Color::RGB(50, 50, 30));
            self.canvas.clear();

            self.canvas.set_draw_color(Color::RGB(230, 255, 255));
            self.render_game_screen(screen);

            self.canvas.present();
        }
    }

    /// create a bunch of point to represent the chip8 screen
    fn render_game_screen(&mut self, arr: &[bool]) {
        let (width, height) = (64, 32);
        // EXTRA UNSAFE
        for y in 0..height {
            for x in 0..width {
                match arr[x + y * width] {
                    true => self
                        .canvas
                        .draw_point(Point::new(x as i32, y as i32))
                        .unwrap(),
                    false => continue,
                };
            }
        }
    }

    /// check which keys were pressed
    /// return true if the user want to exit
    pub fn handle_event(&mut self, keys: &mut [bool]) -> bool {
        // clear all keys
        for k in keys.iter_mut() {
            *k = false;
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => keys[0] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => keys[1] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => keys[2] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => keys[3] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keys[4] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => keys[5] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => keys[6] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => keys[7] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => keys[8] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keys[9] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keys[10] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => keys[11] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keys[12] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => keys[13] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => keys[14] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => keys[15] = true,
                _ => {}
            }
        }
        false
    }
}
