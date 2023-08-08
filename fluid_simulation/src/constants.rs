use sdl2::pixels::Color;

pub const FPS: u64 = 30;
pub const DELTATIME: f64 = 1.0 / FPS as f64;
// pub const WINDOW_SIZE: (u32, u32) = (800, 600);
pub const WINDOW_SIZE: (u32, u32) = (300, 300);

// pub const GRAVITY: f32 = 98.1;
pub const GRAVITY: f32 = 400.0;
pub const DRAG_COEFFICIENT: f32 = 0.2;

pub const PARTICLE_AMT: u32 = 500;
pub const PARTICLE_RADIUS: u32 = 3;
