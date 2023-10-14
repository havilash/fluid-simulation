use crate::constants;

use super::{particle::Particle, vector::Vector};

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

    pub fn query_radius(&self, point: Vector, radius: f32) -> Vec<Particle> {
        let mut neighbors = Vec::new();
        let min = ((point - Vector::new(radius, radius)) / self.cell_size).max(0.0);
        let mut max = (point + Vector::new(radius, radius)) / self.cell_size;
        max.x = max.x.min((self.cells.len() - 1) as f32);
        max.y = max.y.min((self.cells[0].len() - 1) as f32);

        for x in (min.x as usize)..=(max.x as usize) {
            for y in (min.y as usize)..=(max.y as usize) {
                for &index in &self.cells[x][y] {
                    neighbors.push(self.particles[index]);
                }
            }
        }
        neighbors
    }

    pub fn query_all(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn copy(&self) -> ParticlesLookup {
        ParticlesLookup {
            particles: self.particles.clone(),
            cells: self.cells.clone(),
            cell_size: self.cell_size,
        }
    }
}
