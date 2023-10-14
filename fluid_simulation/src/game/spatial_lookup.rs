use crate::constants;

use super::{particle::Particle, vector::Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialLookup {
    cells: Vec<Vec<Vec<usize>>>, // Change this to store indices
    cell_size: f32,
}

impl SpatialLookup {
    pub fn new(cell_size: f32, dimensions: (usize, usize)) -> SpatialLookup {
        let cells = vec![vec![Vec::new(); dimensions.1]; dimensions.0];
        SpatialLookup { cells, cell_size }
    }

    pub fn add_particle(&mut self, particle_index: usize, particle: &Particle) {
        let cell = particle.position / self.cell_size;
        let x = cell.x as usize;
        let y = cell.y as usize;
        self.cells[x][y].push(particle_index);
    }

    pub fn query_radius(
        &self,
        point: Vector,
        radius: f32,
        particles: [Particle; constants::PARTICLE_AMT],
    ) -> Vec<Particle> {
        let mut neighbors = Vec::new();
        let min = ((point - Vector::new(radius, radius)) / self.cell_size).max(0.0);
        let mut max = (point + Vector::new(radius, radius)) / self.cell_size;
        max.x = max.x.min((self.cells.len() - 1) as f32);
        max.y = max.y.min((self.cells[0].len() - 1) as f32);

        for x in (min.x as usize)..=(max.x as usize) {
            for y in (min.y as usize)..=(max.y as usize) {
                for &index in &self.cells[x][y] {
                    neighbors.push(particles[index]);
                }
            }
        }
        neighbors
    }
}
