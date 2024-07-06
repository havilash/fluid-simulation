// https://blog.logrocket.com/using-sdl2-bindings-rust/

extern crate sdl2;

use game::particle::Particle;
use game::vector::Vector;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::video::Window;
use sdl2::{event::Event, rect::Point};
use std::time::{Duration, Instant};

mod constants;
mod game;
use game::game_context::GameContext;

use crate::game::cursor::{self, Cursor, CursorForceType};
use crate::game::game_context::GameState;

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

        for particle in context.particles_lookup.particles.iter() {
            self.draw_circle(
                particle.position.try_into().unwrap(),
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
            let x1 = x.checked_sub(dx);
            let x2 = x.checked_add(dx);
            if let (Some(x1), Some(x2)) = (x1, x2) {
                if let Some(y_dy) = y.checked_add(dy) {
                    self.canvas.draw_line((x1, y_dy), (x2, y_dy)).unwrap();
                }
            }
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
                    pos.try_into().unwrap(),
                    size,
                    Color::RGBA(255, 0, 0, alpha as u8),
                );
            }
        }
        Ok(())
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

    let mut context = GameContext::new(true, 8);
    context.update_heatmap();

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut show_heatmap = false;

    let mut cursor = Cursor::new(
        Vector::new(-1.0, -1.0),
        CursorForceType::None,
        constants::CURSOR_RADIUS,
    );

    let mut step_frame = false;
    let mut frame_count = 0;
    let mut fps_time = Instant::now();
    let mut fps = constants::FPS as f32;
    let mut delta_time = 0.0;
    const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / constants::FPS);
    'running: loop {
        let frame_start = Instant::now();
        frame_count += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::Escape, _) => break 'running,
                    (Keycode::Space, _) => context.toggle_pause(),
                    (Keycode::R, keymod) if keymod.contains(Mod::LSHIFTMOD) => context.reset(false),
                    (Keycode::R, _) => context.reset(true),
                    (Keycode::H, _) => show_heatmap = true,
                    (Keycode::Right, _) => step_frame = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::H, _) => show_heatmap = false,
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    cursor.position.x = x as f32;
                    cursor.position.y = y as f32;
                }
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    mouse::MouseButton::Left => cursor.force_type = CursorForceType::Attract,
                    mouse::MouseButton::Right => cursor.force_type = CursorForceType::Repel,
                    _ => {}
                },
                Event::MouseButtonUp { .. } => cursor.force_type = CursorForceType::None,
                Event::MouseWheel { y, .. } => {
                    cursor.radius += y as f32 * 10.0;
                    cursor.radius.max(0.0);
                }
                _ => {}
            }
        }

        if context.state == GameState::Playing || step_frame {
            context.update(cursor, delta_time);
            step_frame = false;
        }

        if show_heatmap {
            context.update_heatmap();
        }

        if let Err(e) = renderer.draw(&context, show_heatmap) {
            eprintln!("An error occurred while drawing: {}", e);
        }

        let elapsed = fps_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            fps = frame_count as f32 / elapsed.as_secs_f32();
            delta_time = if fps != 0.0 {
                1.0 / fps.max(constants::FPS as f32)
            } else {
                1.0 / constants::FPS as f32
            };

            frame_count = 0;
            fps_time = Instant::now();
        }

        let frame_time = Instant::now() - frame_start;
        if frame_time < FRAME_DURATION {
            ::std::thread::sleep(FRAME_DURATION - frame_time);
        }
    }

    Ok(())
}
