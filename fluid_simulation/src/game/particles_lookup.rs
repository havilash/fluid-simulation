use rand::seq::index;

use crate::constants;

use super::{
    particle::{self, Particle},
    vector::Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParticlesLookup {
    pub particles: Vec<Particle>,
    cells: Vec<Vec<Vec<usize>>>,
    cell_size: f32,
}

impl ParticlesLookup {
    pub fn new(
        particles: Vec<Particle>,
        cell_size: f32,
        dimensions: (usize, usize),
    ) -> ParticlesLookup {
        let cells = vec![vec![Vec::new(); dimensions.1]; dimensions.0];
        ParticlesLookup {
            particles,
            cells,
            cell_size,
        }
    }

    pub fn update_cells(&mut self) {
        for row in &mut self.cells {
            for col in row {
                col.clear();
            }
        }

        for (index, particle) in self.particles.iter().enumerate() {
            let cell = particle.position / self.cell_size;
            let x = cell.x.clamp(0.0, (self.cells.len() - 1) as f32) as usize;
            let y = cell.y.clamp(0.0, (self.cells[0].len() - 1) as f32) as usize;
            self.cells[x][y].push(index);
        }
    }

    pub fn query_radius(
        &mut self,
        point: Option<Vector>,
        radius: Option<f32>,
        current_index: Option<usize>,
    ) -> (Vec<Particle>, Option<&mut Particle>) {
        let mut neighbors: Vec<Particle> = Vec::new();
        let mut current_particle: Option<&mut Particle> = None;
        let mut point = point;
        let mut radius = radius;
        let particles_clone = self.particles.clone();

        if let Some(current_index) = current_index {
            if let Some(particle) = self.particles.get_mut(current_index) {
                let particle_clone = particle.clone();
                current_particle = Some(particle);
                if point.is_none() {
                    point = Some(particle_clone.position);
                }
                if radius.is_none() {
                    radius = Some(particle_clone.get_smoothing_radius());
                }
            }
        }

        let point = point.expect("Point must be set");
        let radius = radius.expect("Radius must be set");

        let min = ((point - Vector::new(radius, radius)) / self.cell_size).max(0.0);
        let mut max = (point + Vector::new(radius, radius)) / self.cell_size;
        max.x = max.x.min((self.cells.len() - 1) as f32);
        max.y = max.y.min((self.cells[0].len() - 1) as f32);

        for x in (min.x as usize)..=(max.x as usize) {
            for y in (min.y as usize)..=(max.y as usize) {
                for &index in &self.cells[x][y] {
                    if let Some(current_index) = current_index {
                        if index == current_index {
                            continue;
                        }
                    }
                    neighbors.push(particles_clone[index].clone());
                }
            }
        }

        (neighbors, current_particle)
    }

    pub fn query_all(
        &mut self,
        current_index: Option<usize>,
    ) -> (Vec<Particle>, Option<&mut Particle>) {
        let mut particles = Vec::new();
        let mut current_particle = None;

        for (index, particle) in self.particles.iter_mut().enumerate() {
            if let Some(current_index) = current_index {
                if index == current_index {
                    current_particle = Some(particle);
                    continue;
                }
            }
            particles.push(particle.clone());
        }

        (particles, current_particle)
    }

    pub fn copy(&self) -> ParticlesLookup {
        ParticlesLookup {
            particles: self.particles.clone(),
            cells: self.cells.clone(),
            cell_size: self.cell_size,
        }
    }
}
