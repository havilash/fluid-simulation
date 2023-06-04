use sdl2::pixels::Color;

use crate::constants;
use crate::game::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Point,
}

impl Particle {
    pub const RADIUS: u32 = constants::PARTICLE_RADIUS;
    pub const COLOR: Color = constants::PARTICLE_COLOR;

    pub fn new(pos: (i32, i32)) -> Particle {
        Particle {
            position: Point::new(pos.0, pos.1),
        }
    }

    pub fn default() -> Particle {
        Particle::new((0, 0))
    }
}
