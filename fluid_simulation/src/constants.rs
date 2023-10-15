use sdl2::pixels::Color;

pub const FPS: u64 = 30;
pub const DELTATIME: f64 = 1.0 / FPS as f64;
pub const WINDOW_SIZE: (u32, u32) = (800, 600);

pub const GRAVITY: f32 = 64.0;
pub const COLLISION_DAMPING: f32 = 0.95;
pub const DRAG_COEFFICIENT: f32 = 0.1;

pub const PARTICLE_AMT: usize = 1028;
pub const PARTICLE_RADIUS: u32 = 4;
pub const PARTICLE_SPACING: u32 = 12;

pub const SMOOTHING_RADIUS: u32 = 32;
pub const DENSITY_FLOOR: f32 = 0.1;
pub const PRESSURE_CONSTANT: f32 = 500.0;
