// https://blog.logrocket.com/using-sdl2-bindings-rust/

extern crate sdl2;

use game::particle::Particle;
use game::vector::Vector;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::video::Window;
use sdl2::{event::Event, rect::Point};
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

    pub fn draw(&mut self, context: &GameContext, show_heatmap: bool) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_blend_mode(BlendMode::Blend);

        if show_heatmap {
            self.draw_heatmap(context)?;
        }

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

    pub fn draw_rect(&mut self, pos: (i32, i32), size: (u32, u32), color: Color) {
        let rect = Rect::new(pos.0, pos.1, size.0, size.1);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_heatmap(&mut self, context: &GameContext) -> Result<(), String> {
        let res = context.heatmap_resolution as f32; // Convert to i32
        for x in 0..context.heatmap.len() {
            for y in 0..context.heatmap[0].len() {
                let alpha = context.heatmap[x][y] * 255.0 * 100.0;
                let pos = Vector::new(x as f32, y as f32) * res - res / 2.0;
                let size = (res as u32, res as u32);

                self.draw_rect(
                    pos.as_tuple_i32(),
                    size,
                    Color::RGBA(255, 0, 0, alpha as u8),
                );
            }
        }
        Ok(())
    }

    pub fn draw_arrow(&mut self, start: (i32, i32), end: (i32, i32), color: Color) {
        // Draw the line part of the arrow
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(start, end).unwrap();

        // Calculate the direction vector of the arrow
        let direction = Vector::new(end.0 as f32 - start.0 as f32, end.1 as f32 - start.1 as f32);
        let unit_direction = direction.normalize();
        let perpendicular = Vector::new(-unit_direction.y, unit_direction.x) * 10.0; // Adjust the size of the arrowhead
        let end_vec = Vector::new(end.0 as f32, end.1 as f32);

        // Draw the arrowhead (two lines at an angle to the main line)
        let arrowhead1 = (end_vec + unit_direction + perpendicular).as_tuple_i32();
        let arrowhead2 = (end_vec + unit_direction - perpendicular).as_tuple_i32();
        self.canvas.draw_line(end, arrowhead1).unwrap();
        self.canvas.draw_line(end, arrowhead2).unwrap();
    }

    pub fn draw_arrow_with_angle(
        &mut self,
        pos: (i32, i32),
        angle: f32,
        length: f32,
        color: Color,
    ) {
        // Calculate the end point of the arrow
        let end_x = pos.0 as f32 + angle.cos() * length;
        let end_y = pos.1 as f32 - angle.sin() * length; // Negative sin because y-axis is flipped in SDL

        // Draw the arrow
        self.draw_arrow(pos, (end_x as i32, end_y as i32), color);
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            constants::WINDOW_SIZE.0,
            constants::WINDOW_SIZE.1,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut context = GameContext::new(true, 10);
    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut show_heatmap = false;

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
                    Keycode::Escape => break 'running,
                    Keycode::Space => context.toggle_pause(),
                    Keycode::R => context.reset(),
                    Keycode::LShift => show_heatmap = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::LShift => show_heatmap = false,
                    _ => {}
                },
                _ => {}
            }
        }

        context.update(show_heatmap);

        if let Err(e) = renderer.draw(&context, show_heatmap) {
            eprintln!("An error occurred while drawing: {}", e);
        }

        let frame_time = Instant::now() - frame_start;
        if frame_time < FRAME_DURATION {
            ::std::thread::sleep(FRAME_DURATION - frame_time);
        }
    }

    Ok(())
}
