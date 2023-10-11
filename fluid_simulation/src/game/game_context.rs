use std::f32::consts::PI;

use rand::Rng;

use crate::constants::{self, WINDOW_SIZE};
use crate::game::utils::calculate_density;

use crate::game::particle::Particle;

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

        GameContext {
            state: GameState::Paused,
            particles: particles,
            frame_count: 0,
            heatmap: vec![vec![0.0; heatmap_height]; heatmap_width],
            heatmap_resolution: heatmap_resolution,
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
                Particle::new(pos.as_tuple_i32(), (0.0, 0.0))
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn create_particles_random_pos() -> [Particle; constants::PARTICLE_AMT as usize] {
        let window_size = constants::WINDOW_SIZE;
        let mut particles = [Particle::new((0, 0), (0.0, 0.0)); constants::PARTICLE_AMT as usize];
        let mut rng = rand::thread_rng();

        for i in 0..constants::PARTICLE_AMT {
            let pos = loop {
                let x = rng.gen_range(0..window_size.0) as f32;
                let y = rng.gen_range(0..window_size.1) as f32;

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

        let particles = &mut self.particles;
        for i in 0..particles.len() {
            let (others, current_and_rest) = particles.split_at_mut(i);
            let (current, rest) = current_and_rest.split_first_mut().unwrap();
            let other_particles: &[Particle] = &[others, rest].concat();
            current.update(true, Some(other_particles));
        }

        if show_heatmap {
            for x in 0..self.heatmap.len() {
                for y in 0..self.heatmap[0].len() {
                    self.heatmap[x][y] = calculate_density(
                        Vector::new(x as f32, y as f32) * self.heatmap_resolution as f32,
                        &self.particles,
                    );
                }
            }
            print!("{:?}", self.heatmap)
        }
    }

    pub fn reset(&mut self) {}

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }
}
