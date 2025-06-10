# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Voxel-Sampo is a 3D voxel scene viewer built with the Bevy game engine. It loads MagicaVoxel (.vox) files and provides an interactive 3D environment with physics-based player movement and camera controls.

## Development Commands

### Build and Run
```bash
# Run the application
cargo run

# Build only
cargo build

# Build for release
cargo build --release
```

### Code Quality
```bash
# Format code
cargo fmt

# Check code
cargo check

# Run clippy for lints
cargo clippy
```

## Architecture

### Core Technologies
- **Bevy 0.16.0**: ECS game engine with dynamic linking for faster compile times
- **bevy_vox_scene 0.19.0**: MagicaVoxel file loading and rendering
- **bevy_rapier3d 0.30.0**: Physics simulation with collision detection

### Module Structure
- `config/`: Centralized configuration system with keybindings and gameplay parameters
- `spawn/`: Entity spawning and scene setup
- `player/`: Player movement components and systems using force-based physics
- `camera/`: Multi-component camera system with rotation, zoom, and snap-to-angle functionality
- `debug/`: Development tools including FPS counter and physics visualization

### Key Architecture Patterns
- **Plugin-based Architecture**: Each module provides a plugin that configures its systems
- **Configuration-Driven**: Tunable parameters (movement force, camera distance, etc.) in centralized config
- **Physics Integration**: Uses Rapier3D with TriMesh colliders generated from voxel geometry
- **Camera-Relative Movement**: Player movement adapts to current camera orientation

### Controls
- WASD: Move player
- Space: Jump
- Q/E: Rotate camera counter-clockwise/clockwise
- F3: Toggle debug mode

### Asset Loading
- Place .vox files in `assets/` directory  
- Voxel models are automatically scaled by 0.05x and generate collision meshes
- HDR environment maps supported for realistic lighting

## Physics System Implementation

### Jump Mechanics
- **Ground Detection**: Uses Y-axis velocity and height to determine if player is grounded
- **Jump Force**: `PLAYER_JUMP_FORCE = 50.0` provides appropriate jump height
- **Air Control Disabled**: Movement forces only apply when grounded for realistic physics
- **Rotation Lock**: `LockedAxes::ROTATION_LOCKED` prevents player from rolling

### Dynamic Damping System
- **Ground Damping**: `linear_damping = 8.0` when grounded for responsive stopping
- **Air Damping**: `linear_damping = 0.1` in air for natural ballistic trajectories
- **System Order**: `ground_detection_system` → `dynamic_damping_system` → `player_input_system`

### Key Physics Parameters
```rust
// Movement and Jump
pub const PLAYER_MOVE_FORCE: f32 = 80.0;
pub const PLAYER_MAX_SPEED: f32 = 4.0;
pub const PLAYER_JUMP_FORCE: f32 = 50.0;
pub const GRAVITY: f32 = 9.8;

// Damping (applied dynamically)
Ground: linear_damping = 8.0
Air: linear_damping = 0.1
```

### Rapier3D 0.30.0 Notes
- `RapierConfiguration` is not a Resource in version 0.30.0
- Use `GravityScale(1.0)` on entities rather than global gravity modification
- Ground detection simplified to velocity + height check for performance