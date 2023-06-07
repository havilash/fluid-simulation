// https://blog.logrocket.com/using-sdl2-bindings-rust/

extern crate sdl2;

use game::particle::Particle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

mod constants;
mod game;
use game::game_context::GameContext;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for particle in context.particles.iter() {
            self.draw_circle(
                particle.position.as_tuple_i32(),
                Particle::RADIUS,
                Particle::COLOR,
            )
        }

        self.canvas.present();
        Ok(())
    }

    pub fn draw_circle(&mut self, center: (i32, i32), radius: u32, color: Color) {
        let (x, y) = center;
        let radius = radius as f64;
        self.canvas.set_draw_color(color);
        for dy in (-radius as i32)..=(radius as i32) {
            let dx = (radius.powi(2) - (dy as f64).powi(2)).sqrt() as i32; // dx^2 + dy^2 = radius^2
            let x1 = x - dx;
            let x2 = x + dx;
            self.canvas.draw_line((x1, y + dy), (x2, y + dy)).unwrap();
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / constants::FPS);
    'running: loop {
        let frame_start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::R => context.reset(),
                    Keycode::Escape => context.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        context.update();

        if let Err(e) = renderer.draw(&context) {
            eprintln!("An error occurred while drawing: {}", e);
        }

        let frame_time = Instant::now() - frame_start;
        if frame_time < FRAME_DURATION {
            ::std::thread::sleep(FRAME_DURATION - frame_time);
        }
    }

    Ok(())
}
