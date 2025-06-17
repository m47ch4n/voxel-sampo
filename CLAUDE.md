# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**⚠️ IMPORTANT: This documentation must be kept up-to-date with code changes. Outdated documentation can lead to incorrect assumptions and poor development decisions. Always verify and update this file when implementing new features or refactoring existing systems.**

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
- `physics/`: Physics simulation with raycast-based ground detection and dynamic damping
- `debug/`: Comprehensive development tools with 8 specialized modules for runtime diagnostics

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

### Ground Detection System (Raycast-Based)
- **Implementation**: Uses raycast from player bottom to detect ground contact
- **Ray Parameters**: 
  - Origin: 2cm above player collider bottom (`GROUND_RAY_OFFSET_EPS = 2e-2`)
  - Distance: 10cm downward (`GROUND_RAY_DISTANCE = 1e-1`)
  - Excludes: Player's own collider from raycast
- **Component**: `GroundedState` tracks ray info and grounded status with hit point data

### Jump Mechanics
- **Jump Force**: `PLAYER_JUMP_FORCE = 50.0` provides appropriate jump height
- **Air Control**: Limited air movement with `PLAYER_AIR_CONTROL_FORCE = 2.0`
- **Ground Movement**: Full movement force `PLAYER_MOVE_FORCE = 80.0` when grounded
- **Rotation Lock**: `LockedAxes::ROTATION_LOCKED` prevents player from rolling

### Dynamic Damping System
- **Ground Damping**: `linear_damping = 8.0` when grounded for responsive stopping
- **Air Damping**: `linear_damping = 0.1` in air for natural ballistic trajectories
- **Component**: `DynamicDamping` manages ground/air damping values
- **System Order**: `ground_detection_system` → `dynamic_damping_system` → `player_input_system`

### Key Physics Parameters
```rust
// Movement Forces
pub const PLAYER_MOVE_FORCE: f32 = 80.0;
pub const PLAYER_AIR_CONTROL_FORCE: f32 = 2.0;
pub const PLAYER_MAX_SPEED: f32 = 4.0;
pub const PLAYER_JUMP_FORCE: f32 = 50.0;

// Ground Detection
const GROUND_RAY_OFFSET_EPS: f32 = 2e-2;  // 2cm offset
const GROUND_RAY_DISTANCE: f32 = 1e-1;    // 10cm ray distance

// Dynamic Damping
Ground: linear_damping = 8.0
Air: linear_damping = 0.1
```

## Debug System

### Core Features
- **Toggle**: F3 key to enable/disable debug mode
- **Visual Elements**: Debug UI overlay with ASCII art header and version info
- **Physics Debug**: Rapier3D debug rendering integration

### Information Modules (8 specialized systems)
- **FPS**: Real-time frame rate monitoring
- **Performance**: Entity count, RigidBody count tracking
- **Player**: 3D coordinates, block coordinates, velocity magnitude
- **Physics**: Ground state, ray info, damping values, gravity scale, friction, restitution
- **World**: Camera position, rotation, and zoom information
- **Ray Visualization**: Ground detection ray (green=grounded, red=airborne)

### Debug Components
- `DebugState`: Global debug state management
- UI panels with semi-transparent backgrounds for readability

## Rapier3D 0.30.0 Notes
- `RapierConfiguration` is not a Resource in version 0.30.0
- Use `GravityScale(1.0)` on entities rather than global gravity modification
- Ground detection uses precise raycast with epsilon-based positioning for accuracy