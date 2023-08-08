use sdl2::pixels::Color;

use crate::constants;
use crate::game::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
}

impl Particle {
    pub const RADIUS: u32 = constants::PARTICLE_RADIUS;
    pub const COLOR: Color = Color::BLUE;

    pub fn new(position: (i32, i32), velocity: (f32, f32)) -> Particle {
        Particle {
            position: Vector::new(position.0 as f32, position.1 as f32),
            velocity: Vector::new(velocity.0, velocity.1),
        }
    }

    pub fn update(&mut self, particles: Option<&[&Particle]>) {
        let acceleration = self.calculate_acceleration();

        let mut final_velocity = self.velocity + acceleration * (constants::DELTATIME as f32);
        let particles = particles.unwrap_or(&[]);
        let mut normal = self.collide(particles);
        self.reflect(final_velocity, normal)
    }

    fn collide_bounding(&self) -> Vector {
        let new_position = self.next_position();
        let mut normal = Vector::zero();
        if (new_position.x - Particle::RADIUS as f32) < 0.0 {
            normal += Vector::new(1.0, 0.0);
        } else if (new_position.x + Particle::RADIUS as f32) > constants::WINDOW_SIZE.0 as f32 {
            normal += Vector::new(-1.0, 0.0);
        }
        if (new_position.y - Particle::RADIUS as f32) < 0.0 {
            normal += Vector::new(0.0, 1.0);
        } else if (new_position.y + Particle::RADIUS as f32) > constants::WINDOW_SIZE.1 as f32 {
            normal += Vector::new(0.0, -1.0);
        }
        return normal.normalize();
    }

    fn collide_particle(&self, other: &Particle) -> bool {
        let distance = (self.next_position() - other.next_position()).magnitude();
        if distance < (Particle::RADIUS * 2) as f32 {
            return true;
        }
        return false;
    }

    fn collide(&self, particles: &[&Particle]) -> Vector {
        let mut normal = Vector::zero();
        for particle in particles {
            if self.collide_particle(particle) {
                normal += (self.position - particle.position).normalize();
            }
        }
        normal += self.collide_bounding();
        return normal.normalize();
    }

    fn reflect(&mut self, final_velocity: Vector, normal: Vector) {
        let new_final_velocity = final_velocity - normal * 2.0 * final_velocity.dot(normal);
        let displacement = new_final_velocity * (constants::DELTATIME as f32);
        let new_position = self.position + displacement;
        self.velocity = new_final_velocity;
        self.position = new_position;

        self.position.x = new_position.x.max(Particle::RADIUS as f32);
        self.position.x = self
            .position
            .x
            .min(constants::WINDOW_SIZE.0 as f32 - Particle::RADIUS as f32);
        self.position.y = new_position.y.max(Particle::RADIUS as f32);
        self.position.y = self
            .position
            .y
            .min(constants::WINDOW_SIZE.1 as f32 - Particle::RADIUS as f32);
    }

    fn next_position(&self) -> Vector {
        let acceleration = self.calculate_acceleration();

        let final_velocity = self.velocity + acceleration * (constants::DELTATIME as f32);
        let displacement = (final_velocity + self.velocity) / 2.0 * (constants::DELTATIME as f32);
        let new_position = self.position + displacement;
        return new_position;
    }

    fn calculate_acceleration(&self) -> Vector {
        let acceleration_gravity = Vector::new(0.0, 1.0) * (constants::GRAVITY as f32);
        let drag_coefficient = constants::DRAG_COEFFICIENT;
        let drag_force =
            self.velocity.normalize() * -drag_coefficient * self.velocity.magnitude().powi(2);

        let acceleration = acceleration_gravity + drag_force;
        return acceleration;
    }
}
