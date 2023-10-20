use std::f32::consts::PI;

use rand::Rng;
use sdl2::pixels::Color;

use crate::constants;
use crate::game::utils::calculate_density;
use crate::game::vector::Vector;

use super::{
    cursor::{self, Cursor, CursorForceType},
    utils::{
        calculate_shared_pressure, random_direction, smoothing_kernel, smoothing_kernel_derivative,
    },
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub density: f32,
}

impl Particle {
    pub const RADIUS: u32 = constants::PARTICLE_RADIUS;
    pub const SMOOTHING_RADIUS: u32 = constants::SMOOTHING_RADIUS;
    pub const COLOR: Color = Color::BLUE;
    pub const MASS: f32 = 1.0;

    pub fn new(position: (i32, i32), velocity: (f32, f32)) -> Particle {
        Particle {
            position: Vector::new(position.0 as f32, position.1 as f32),
            velocity: Vector::new(velocity.0, velocity.1),
            density: 0.0,
        }
    }

    pub fn update(&mut self, use_gravity: bool, other_particles: &Vec<Particle>, cursor: Cursor) {
        self.density = self.calculate_density(self.position, other_particles);
        let acceleration = self.calculate_acceleration(use_gravity, other_particles, cursor);

        let final_velocity = self.velocity + acceleration * (constants::DELTATIME as f32);
        let normal = self.collide(acceleration);
        self.reflect(final_velocity, normal);
    }

    fn collide_bounding(&self, position: Vector) -> Vector {
        let mut normal = Vector::zero();
        if (position.x - Self::RADIUS as f32) < 0.0 {
            normal += Vector::new(1.0, 0.0);
        } else if (position.x + Self::RADIUS as f32) > constants::WINDOW_SIZE.0 as f32 {
            normal += Vector::new(-1.0, 0.0);
        }
        if (position.y - Self::RADIUS as f32) < 0.0 {
            normal += Vector::new(0.0, 1.0);
        } else if (position.y + Self::RADIUS as f32) > constants::WINDOW_SIZE.1 as f32 {
            normal += Vector::new(0.0, -1.0);
        }
        return normal;
    }

    fn collide(&self, acceleration: Vector) -> Vector {
        let mut normal = Vector::zero();
        let new_position = self.next_position(acceleration);
        normal += self.collide_bounding(new_position);
        return normal.normalize() * constants::COLLISION_DAMPING;
    }

    fn reflect(&mut self, final_velocity: Vector, normal: Vector) {
        let new_final_velocity = final_velocity - normal * 2.0 * final_velocity.dot(normal);
        let displacement = new_final_velocity * (constants::DELTATIME as f32);

        let new_position = self.position + displacement;

        self.velocity = new_final_velocity;
        self.position = new_position;
    }

    fn calculate_acceleration(
        &self,
        use_gravity: bool,
        other_particles: &Vec<Particle>,
        cursor: Cursor,
    ) -> Vector {
        let mut acceleration = Vector::zero();

        if use_gravity {
            let acceleration_gravity = Vector::new(0.0, 1.0) * (constants::GRAVITY as f32);
            acceleration += acceleration_gravity;
        }

        let drag_coefficient = constants::DRAG_COEFFICIENT;
        let drag_force =
            self.velocity.normalize() * -drag_coefficient * self.velocity.magnitude().powi(2);
        acceleration += drag_force / Self::MASS;

        let pressure_force = self.calculate_pressure_force(other_particles);
        acceleration += pressure_force / (self.density + 1e-3);

        let cursor_force = self.calculate_cursor_force(cursor);
        acceleration += cursor_force / (self.density + 1e-3);

        acceleration
    }

    fn next_position(&self, acceleration: Vector) -> Vector {
        let final_velocity = self.velocity + acceleration * (constants::DELTATIME as f32);
        let displacement = final_velocity * (constants::DELTATIME as f32);
        let new_position = self.position + displacement;
        return new_position;
    }

    fn calculate_cursor_force(&self, cursor: Cursor) -> Vector {
        if cursor.force_type == CursorForceType::None {
            return Vector::zero();
        }
        let offset = cursor.position - self.position;
        let dst = offset.magnitude();
        let dir = if dst == 0.0 {
            random_direction()
        } else {
            offset / dst
        };
        let mut force = Vector::zero();
        if dst < cursor.radius as f32 {
            match cursor.force_type {
                CursorForceType::Attract => {
                    force = dir;
                }
                CursorForceType::Repel => {
                    force = dir * -1.0;
                }
                _ => {}
            }
        }
        return force * constants::CURSOR_CONSTANT;
    }

    fn calculate_pressure_force(&self, other_particles: &Vec<Particle>) -> Vector {
        let mut pressure_force: Vector = Vector::zero();

        for other in other_particles {
            if self == other {
                continue;
            }

            let offset = other.position - self.position;
            let dst = offset.magnitude();
            let dir = if dst == 0.0 {
                random_direction()
            } else {
                offset / dst
            };

            let slope = smoothing_kernel_derivative(dst, self.get_smoothing_radius());
            let pressure = calculate_shared_pressure(self.density, other.density);
            pressure_force -= dir * pressure * slope;
        }
        return pressure_force;
    }

    pub fn calculate_density(&self, point: Vector, other_particles: &Vec<Particle>) -> f32 {
        let mut density = 0.0;
        for p in other_particles {
            if self == p {
                continue;
            }
            let dst = (p.position - point).magnitude();
            let influence = smoothing_kernel(dst, p.get_smoothing_radius());
            density += Particle::MASS * influence;
        }
        return density;
    }

    pub fn get_smoothing_radius(&self) -> f32 {
        Particle::SMOOTHING_RADIUS as f32
    }
}
