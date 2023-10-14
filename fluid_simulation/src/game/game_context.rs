use std::f32::consts::PI;

use rand::Rng;

use crate::constants::{self, WINDOW_SIZE};
use crate::game::utils::calculate_density;

use crate::game::particle::Particle;

use super::spatial_lookup::SpatialLookup;
use super::vector::Vector;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
}

pub struct GameContext {
    pub state: GameState,
    pub particles: [Particle; constants::PARTICLE_AMT as usize],
    pub frame_count: u32,
    pub heatmap: Vec<Vec<f32>>,
    pub heatmap_resolution: u32,
    pub spatial_lookup: SpatialLookup,
}

impl GameContext {
    pub fn new(use_random_pos: bool, heatmap_resolution: u32) -> GameContext {
        let heatmap_width = (constants::WINDOW_SIZE.0 / heatmap_resolution + 1) as usize;
        let heatmap_height = (constants::WINDOW_SIZE.1 / heatmap_resolution + 1) as usize;

        let particles = if use_random_pos {
            Self::create_particles_random_pos()
        } else {
            Self::create_particles_grid()
        };

        let spatial_lookup_size = Particle::SMOOTHING_RADIUS as f32;
        let spatial_lookup_dimensions: (usize, usize) = (Vector::from(constants::WINDOW_SIZE)
            / spatial_lookup_size)
            .ceil()
            .try_into()
            .unwrap();

        GameContext {
            state: GameState::Paused,
            particles: particles,
            frame_count: 0,
            heatmap: vec![vec![0.0; heatmap_height]; heatmap_width],
            heatmap_resolution: heatmap_resolution,
            spatial_lookup: SpatialLookup::new(spatial_lookup_size, spatial_lookup_dimensions),
        }
    }

    fn create_particles_grid() -> [Particle; constants::PARTICLE_AMT as usize] {
        let particle_amt = constants::PARTICLE_AMT;
        let spacing = constants::PARTICLE_SPACING as i32;
        let rows = (particle_amt as f32).sqrt().ceil() as i32;
        let grid_size = Vector::new(
            (rows * spacing) as f32,
            (particle_amt as i32 * spacing / rows) as f32,
        );

        let offset = (Vector::from(constants::WINDOW_SIZE) - grid_size) / 2.0;

        (0..particle_amt)
            .map(|i| {
                let i = i as i32;
                let pos =
                    Vector::new((i % rows) as f32, (i / rows) as f32) * spacing as f32 + offset;
                Particle::new(pos.try_into().unwrap(), (0.0, 0.0))
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn create_particles_random_pos() -> [Particle; constants::PARTICLE_AMT as usize] {
        let window_size = constants::WINDOW_SIZE;
        let radius = Particle::RADIUS;
        let mut particles = [Particle::new((0, 0), (0.0, 0.0)); constants::PARTICLE_AMT as usize];
        let mut rng = rand::thread_rng();

        for i in 0..constants::PARTICLE_AMT {
            let pos = loop {
                // let x = rng.gen_range(0..window_size.0) as f32;
                // let y = rng.gen_range(0..window_size.1) as f32;
                let x = rng.gen_range((radius)..(window_size.0 - radius)) as f32;
                let y = rng.gen_range((radius)..(window_size.1 - radius)) as f32;

                let is_overlap = particles.iter().any(|particle| {
                    let dx = particle.position.x - x;
                    let dy = particle.position.y - y;
                    (dx * dx + dy * dy).sqrt() < Particle::RADIUS as f32 * 2.0
                });

                if !is_overlap {
                    break (x as i32, y as i32);
                }
            };

            particles[i as usize] = Particle::new(pos, (0.0, 0.0));
        }

        particles
    }

    pub fn update(&mut self, show_heatmap: bool) {
        if GameState::Paused == self.state {
            return;
        }

        self.frame_count += 1;
        self.update_spatial_lookup();
        for i in 0..self.particles.len() {
            let (before, current_and_rest) = self.particles.split_at_mut(i);
            let (current, rest) = current_and_rest.split_first_mut().unwrap();
            let other_particles = &[before, rest].concat();

            self.particles[i].update(false, other_particles, &self.spatial_lookup);
        }

        if show_heatmap {
            print!(
                "{:?}",
                self.spatial_lookup
                    .query_radius(Vector::new(0.0, 0.0), 28.0, self.particles)
            );
            for x in 0..self.heatmap.len() {
                for y in 0..self.heatmap[0].len() {
                    self.heatmap[x][y] = calculate_density(
                        Vector::new(x as f32, y as f32) * self.heatmap_resolution as f32,
                        &self.particles,
                        &self.spatial_lookup,
                    );
                }
            }
        }
    }

    pub fn reset(&mut self) {}

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }

    fn update_spatial_lookup(&mut self) {
        for (i, particle) in self.particles.iter().enumerate() {
            self.spatial_lookup.add_particle(i, particle)
        }
    }
}
