use rand::Rng;

use crate::constants::{self};
use crate::game::utils::calculate_density;

use crate::game::particle::Particle;

use super::cursor::Cursor;
use super::particle;
use super::particles_lookup::ParticlesLookup;
use super::vector::Vector;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
}

pub struct GameContext {
    pub state: GameState,
    pub frame_count: u32,
    pub heatmap: Vec<Vec<f32>>,
    pub heatmap_resolution: u32,
    pub particles_lookup: ParticlesLookup,
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

        let particles_lookup_size = Particle::SMOOTHING_RADIUS as f32;
        let particles_lookup_dimensions: (usize, usize) = (Vector::from(constants::WINDOW_SIZE)
            / particles_lookup_size)
            .ceil()
            .try_into()
            .unwrap();

        GameContext {
            state: GameState::Paused,
            frame_count: 0,
            heatmap: vec![vec![0.0; heatmap_height]; heatmap_width],
            heatmap_resolution: heatmap_resolution,
            particles_lookup: ParticlesLookup::new(
                particles,
                particles_lookup_size,
                particles_lookup_dimensions,
            ),
        }
    }

    fn create_particles_grid() -> Vec<Particle> {
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
    }

    fn create_particles_random_pos() -> Vec<Particle> {
        let window_size = constants::WINDOW_SIZE;
        let radius = Particle::RADIUS;
        let mut particles =
            vec![Particle::new((0, 0), (0.0, 0.0)); constants::PARTICLE_AMT as usize];
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

    pub fn update(&mut self, show_heatmap: bool, use_gravity: bool, cursor: Cursor) {
        if GameState::Paused == self.state {
            return;
        }

        self.frame_count += 1;

        self.particles_lookup.update_cells();

        for i in 0..constants::PARTICLE_AMT {
            let (other_particles, current_option) =
                self.particles_lookup.query_radius(None, None, Some(i));
            // let (other_particles, current_option) = self.particles_lookup.query_all(Some(i));
            if let Some(current) = current_option {
                current.update(use_gravity, &other_particles, cursor);
            }
        }

        if show_heatmap {
            self.update_heatmap()
        }
    }

    pub fn update_heatmap(&mut self) {
        for x in 0..self.heatmap.len() {
            for y in 0..self.heatmap[0].len() {
                let point = Vector::new(x as f32, y as f32) * self.heatmap_resolution as f32;
                let (other_particles, _) = self.particles_lookup.query_radius(
                    Some(point),
                    Some(Particle::SMOOTHING_RADIUS as f32),
                    None,
                );

                let density = calculate_density(point, &other_particles);
                self.heatmap[x][y] = density;
            }
        }
    }

    pub fn reset(&mut self, use_random_pos: bool) {
        let particles = if use_random_pos {
            Self::create_particles_random_pos()
        } else {
            Self::create_particles_grid()
        };

        self.particles_lookup.particles = particles;

        self.particles_lookup.update_cells();

        self.update_heatmap();
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }
}
