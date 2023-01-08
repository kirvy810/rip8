use crate::cpu::Cpu;
use crate::video::Video;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub struct App {
    canvas: Canvas<Window>,
    events: EventPump,
    cpu: Cpu,
}

impl App {
    pub fn new(cpu: Cpu) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;

        let width = 15 * Video::WIDTH as u32;
        let height = 15 * Video::HEIGHT as u32;

        let window = video_subsystem
            .window("RIP-8", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let events = context.event_pump().map_err(|e| e.to_string())?;

        Ok(Self {
            canvas,
            events,
            cpu,
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = Self::decode_key(keycode) {
                            self.cpu.keypad.key_down(key);
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = Self::decode_key(keycode) {
                            self.cpu.keypad.key_up(key);
                        }
                    }
                    _ => {}
                }
            }

            self.cpu.cycle();
            self.render_video();
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1000000000 / 60));
        }
    }

    fn render_video(&mut self) {
        for i in 0..Video::HEIGHT {
            for j in 0..Video::WIDTH {
                self.canvas
                    .set_draw_color(if self.cpu.video[i * Video::WIDTH + j] {
                        Color::RGB(3, 252, 78)
                    } else {
                        Color::RGB(20, 20, 20)
                    });

                let x = (15 * j) as i32;
                let y = (15 * i) as i32;

                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x, y, 15, 15));
            }
        }
    }

    fn decode_key(keycode: Option<Keycode>) -> Option<u8> {
        match keycode {
            Some(Keycode::Num1) => Some(0x1),
            Some(Keycode::Num2) => Some(0x2),
            Some(Keycode::Num3) => Some(0x3),
            Some(Keycode::Num4) => Some(0xC),
            Some(Keycode::Q) => Some(0x4),
            Some(Keycode::W) => Some(0x5),
            Some(Keycode::E) => Some(0x6),
            Some(Keycode::R) => Some(0xD),
            Some(Keycode::A) => Some(0x7),
            Some(Keycode::S) => Some(0x8),
            Some(Keycode::D) => Some(0x9),
            Some(Keycode::F) => Some(0xE),
            Some(Keycode::Z) => Some(0xA),
            Some(Keycode::X) => Some(0x0),
            Some(Keycode::C) => Some(0xB),
            Some(Keycode::V) => Some(0xF),
            _ => None,
        }
    }
}
