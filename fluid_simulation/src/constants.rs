use sdl2::pixels::Color;

pub const FPS: u64 = 30;
pub const DELTATIME: f64 = 1.0 / FPS as f64;
pub const WINDOW_SIZE: (u32, u32) = (1200, 900);

pub const GRAVITY: f32 = 128.0;
pub const COLLISION_DAMPING: f32 = 1.0;
pub const DRAG_COEFFICIENT: f32 = 0.1;

pub const PARTICLE_AMT: usize = 4096;
pub const PARTICLE_RADIUS: u32 = 3;
pub const PARTICLE_SPACING: u32 = 7;

pub const SMOOTHING_RADIUS: u32 = 18;
pub const DENSITY_FLOOR: f32 = 20.0;
pub const PRESSURE_CONSTANT: f32 = 500.0;

pub const CURSOR_RADIUS: f32 = 128.0;
pub const CURSOR_CONSTANT: f32 = 10.0;
