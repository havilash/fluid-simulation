use crate::constants;
use rand::Rng;

use crate::game::particle::Particle;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
}

pub struct GameContext {
    pub state: GameState,
    pub particles: [Particle; constants::PARTICLE_AMT as usize],
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut rng = rand::thread_rng();
        let mut particles: [Particle; constants::PARTICLE_AMT as usize] =
            [Particle::default(); constants::PARTICLE_AMT as usize];
        for i in 0..constants::PARTICLE_AMT {
            particles[i as usize].position.x = rng.gen_range(0..(constants::WINDOW_SIZE.0 as i32));
            particles[i as usize].position.y = rng.gen_range(0..(constants::WINDOW_SIZE.0 as i32));
        }

        GameContext {
            state: GameState::Paused,
            particles: particles,
        }
    }

    pub fn next_tick(&mut self) {
        if GameState::Paused == self.state {
            return;
        }
        for particle in self.particles.iter_mut() {
            particle.position.y += 1;
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
