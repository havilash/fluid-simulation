use std::f32::consts::PI;

use rand::Rng;

use crate::constants;

use super::{particle::Particle, particles_lookup::ParticlesLookup, vector::Vector};

pub fn calculate_density(point: Vector, particles_lookup: &ParticlesLookup) -> f32 {
    let mut density = 0.0;
    let neighbors = particles_lookup.query_radius(point, Particle::SMOOTHING_RADIUS as f32);
    for p in neighbors {
        let dst = (p.position - point).magnitude();
        let influence = smoothing_kernel(dst, p.get_smoothing_radius() as f32);
        density += Particle::MASS * influence;
    }
    return density;
}

// TODO: better smoothing kernel (spickier)
pub fn smoothing_kernel(dst: f32, radius: f32) -> f32 {
    if dst >= radius {
        return 0.0;
    }

    let volume = (PI * radius.powi(4)) / 6.0;
    return (radius - dst).powi(2) / volume;
}

pub fn smoothing_kernel_derivative(dst: f32) -> f32 {
    let radius = Particle::SMOOTHING_RADIUS as f32;
    if dst >= radius {
        return 0.0;
    }

    let scale = 12.0 / (radius.powi(4) * PI);
    return (dst - radius) * scale;
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
    let x = rng.gen_range(-1.0..=1.0);
    let y = rng.gen_range(-1.0..=1.0);
    Vector::new(x, y).normalize()
}
