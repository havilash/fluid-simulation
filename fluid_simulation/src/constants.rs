use sdl2::pixels::Color;

pub const FPS: u64 = 30;
pub const DELTATIME: f64 = 1.0 / FPS as f64;
pub const WINDOW_SIZE: (u32, u32) = (400, 300);

pub const GRAVITY: f32 = 9.81;
pub const COLLISION_DAMPING: f32 = 0.95;

pub const PARTICLE_AMT: u32 = 512;
pub const PARTICLE_RADIUS: u32 = 2;
pub const PARTICLE_SPACING: u32 = 16;

pub const SMOOTHING_RADIUS: u32 = 20;
pub const DENSITY_FLOOR: f32 = 2.0;
pub const PRESSURE_CONSTANT: f32 = 0.0;
