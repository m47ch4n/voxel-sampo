use bevy::prelude::*;

pub const PLAYER_MOVE_FORCE: f32 = 80.0;
pub const PLAYER_MAX_SPEED: f32 = 4.0;
pub const PLAYER_JUMP_FORCE: f32 = 50.0;
pub const PLAYER_AIR_CONTROL_FORCE: f32 = 2.0;
pub const CAMERA_DISTANCE: f32 = 40.0;
pub const CAMERA_HEIGHT: f32 = 24.0;
pub const CAMERA_ANGLES: [f32; 4] = [60.0, 150.0, 240.0, 330.0];
pub const BASE_ZOOM: f32 = 10.0;
pub const ZOOM_OUT_VALUE: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub camera_rotate_clockwise: KeyCode,
    pub camera_rotate_counter_clockwise: KeyCode,
    pub player_move_up: KeyCode,
    pub player_move_down: KeyCode,
    pub player_move_left: KeyCode,
    pub player_move_right: KeyCode,
    pub player_jump: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            camera_rotate_clockwise: KeyCode::KeyE,
            camera_rotate_counter_clockwise: KeyCode::KeyQ,
            player_move_up: KeyCode::KeyW,
            player_move_down: KeyCode::KeyS,
            player_move_left: KeyCode::KeyA,
            player_move_right: KeyCode::KeyD,
            player_jump: KeyCode::Space,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerConfig {
    pub move_force: f32,
    pub max_speed: f32,
    pub jump_force: f32,
    pub air_control_force: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            move_force: PLAYER_MOVE_FORCE,
            max_speed: PLAYER_MAX_SPEED,
            jump_force: PLAYER_JUMP_FORCE,
            air_control_force: PLAYER_AIR_CONTROL_FORCE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub distance: f32,
    pub height: f32,
    pub base_zoom: f32,
    pub zoom_out_value: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            distance: CAMERA_DISTANCE,
            height: CAMERA_HEIGHT,
            base_zoom: BASE_ZOOM,
            zoom_out_value: ZOOM_OUT_VALUE,
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct Config {
    pub player: PlayerConfig,
    pub camera: CameraConfig,
    pub key_bindings: KeyBindings,
}
