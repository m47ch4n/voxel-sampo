use bevy::prelude::*;

pub const PLAYER_MOVE_SPEED: f32 = 1.0;
pub const PLAYER_MOVE_DURATION: f32 = 0.1;
pub const CAMERA_DISTANCE: f32 = 12.0;
pub const CAMERA_HEIGHT: f32 = 8.0;
pub const CAMERA_ROTATION_SPEED: f32 = 360.0;
pub const CAMERA_ANGLES: [f32; 4] = [60.0, 150.0, 240.0, 330.0];
pub const BASE_ZOOM: f32 = 10.0;
pub const ZOOM_OUT_VALUE: f32 = 15.0;

#[derive(Resource)]
pub struct GameSettings {
    pub player_move_speed: f32,
    pub player_move_duration: f32,
    pub camera_distance: f32,
    pub camera_height: f32,
    pub camera_rotation_speed: f32,
    pub base_zoom: f32,
    pub zoom_out_value: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_move_speed: PLAYER_MOVE_SPEED,
            player_move_duration: PLAYER_MOVE_DURATION,
            camera_distance: CAMERA_DISTANCE,
            camera_height: CAMERA_HEIGHT,
            camera_rotation_speed: CAMERA_ROTATION_SPEED,
            base_zoom: BASE_ZOOM,
            zoom_out_value: ZOOM_OUT_VALUE,
        }
    }
} 