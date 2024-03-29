use std::f32::consts::PI;

use rand::Rng;

use crate::constants;

use super::{particle::Particle, particles_lookup::ParticlesLookup, vector::Vector};

pub fn calculate_density(point: Vector, other_particles: &Vec<Particle>) -> f32 {
    let mut density = 0.0;
    for p in other_particles {
        let dst = (p.position - point).magnitude();
        let influence = smoothing_kernel(dst, p.get_smoothing_radius() as f32);
        density += Particle::MASS * influence;
    }
    return density;
}

// Integrate[(s-x)^3x,{x,0,s},{θ,0,2π}]
pub fn smoothing_kernel(dst: f32, radius: f32) -> f32 {
    if dst >= radius {
        return 0.0;
    }

    let volume = (PI * radius.powi(4)) / 6.0;
    return (radius - dst).powi(2) / volume;
}

pub fn smoothing_kernel_derivative(dst: f32, radius: f32) -> f32 {
    if dst >= radius {
        return 0.0;
    }

    let scale = 12.0 / (radius.powi(4) * PI);
    return (dst - radius) * scale;
}

pub fn viscosity_smoothing_kernel(dst: f32, radius: f32) -> f32 {
    if dst >= radius {
        return 0.0;
    }

    let volume = (PI * radius.powi(8)) / 4.0;
    let value = (radius.powi(2) - dst.powi(2)).max(0.0);
    return value.powi(3) / volume;
}

pub fn density_to_pressure(density: f32) -> f32 {
    return constants::PRESSURE_CONSTANT * (density - constants::DENSITY_FLOOR);
}

pub fn calculate_shared_pressure(density_a: f32, density_b: f32) -> f32 {
    let pressure_a = density_to_pressure(density_a);
    let pressure_b = density_to_pressure(density_b);
    (pressure_a + pressure_b) / 2.0
}

pub fn random_direction() -> Vector {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * PI);
    let x = angle.cos();
    let y = angle.sin();
    Vector::new(x, y)
}
