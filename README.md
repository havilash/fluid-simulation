# Fluid Simulation

A simple Rust-based fluid dynamics simulation using SDL2 for rendering and input. Interact with particles via the mouse and keyboard.

## Setup

### Prerequisites

- **Rust & Cargo:** [Install Rust](https://www.rust-lang.org/tools/install)
- **SDL2 (MSVC version):** Follow the instructions on the [rust-sdl2 README](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) for installing the SDL2 development libraries on Windows.

## Usage

1. **Build & Run:**  
   Open a terminal in the project root (where `Cargo.toml` is located) and execute:
   ```bash
   cargo run
   ```

### Controls

- **Mouse:** Move to interact; left-click attracts, right-click repels.
- **Wheel:** Adjust influence radius.
- **Space:** Pause/resume.
- **R:** Reset (Shift+R for alternate).
- **H:** Toggle heatmap.
- **Right Arrow:** Step one frame.
- **Escape:** Quit.
