use crate::constants;
use rand::Rng;

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
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..constants::PARTICLE_AMT {
            loop {
                // random positions
                let x = rng.gen_range(
                    Particle::RADIUS..(constants::WINDOW_SIZE.0 as u32 - Particle::RADIUS),
                ) as f32;
                let y = rng.gen_range(
                    Particle::RADIUS..(constants::WINDOW_SIZE.1 as u32 - Particle::RADIUS),
                ) as f32;
                let position = Vector::new(x, y);

                // check overlap
                let mut overlaps = false;
                for particle in particles.iter() {
                    if (position - particle.position).magnitude() < (Particle::RADIUS * 2) as f32 {
                        overlaps = true;
                        break;
                    }
                }

                if !overlaps {
                    particles.push(Particle::new(position.as_tuple_i32(), (100.0, -100.0)));
                    break;
                }
            }
        }

        GameContext {
            state: GameState::Playing,
            particles: particles.try_into().unwrap(),
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        if GameState::Paused == self.state {
            return;
        }

        self.frame_count += 1;

        let particles = &mut self.particles;
        for i in 0..particles.len() {
            let (before, rest) = particles.split_at_mut(i);
            let (particle, after) = rest.split_first_mut().unwrap();
            let other_particles: Vec<_> = before.iter().chain(after.iter()).collect();
            particle.update(Some(&other_particles));
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
