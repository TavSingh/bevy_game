# bevy_game

Lightweight Bevy-based game prototype written in Rust.

## Summary
A small game project using the Bevy engine. Intended as a sandbox for learning Bevy patterns, ECS design, and simple game systems.

## Requirements
- Rust (stable) — install via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Cargo (comes with Rust)
- SDL/graphics dependencies as required by your Linux distribution (usually included)

Tested on Linux.

## Quick start
1. Clone the repo:
   - `git clone https://github.com/TavSingh/bevy_game.git`
   - `cd bevy_game`
2. Run in debug:
   - `cargo run`
3. Run optimized:
   - `cargo run --release`

If you hit missing system libs, install your distro's SDL2/graphics packages (e.g., `libsdl2-dev`, `libwayland-dev`, etc.).

## Project layout
- src/ — Rust source files (Bevy app entry, systems, components)
- assets/ — game assets (images, audio) if present
- Cargo.toml — project configuration and dependencies

## Development
- Add systems/components under src/
- Use `cargo fmt` and `cargo clippy` for formatting and linting:
  - `cargo fmt`
  - `cargo clippy -- -D warnings`
