use bevy::prelude::*;
use crate::config::{GameSettings, CAMERA_ANGLES, CAMERA_ROTATION_SPEED, BASE_ZOOM, ZOOM_OUT_VALUE};

#[derive(Component, Debug, Clone)]
pub struct CameraAngle {
    pub current_angle: f32,
}

impl Default for CameraAngle {
    fn default() -> Self {
        Self {
            current_angle: CAMERA_ANGLES[0],
        }
    }
}

impl CameraAngle {
    pub fn get_camera_forward_direction(&self) -> Vec3 {
        let rad = self.current_angle.to_radians();
        Vec3::new(-rad.cos(), 0.0, -rad.sin()).normalize()
    }
    
    pub fn get_camera_right_direction(&self) -> Vec3 {
        let forward = self.get_camera_forward_direction();
        forward.cross(Vec3::Y).normalize()
    }

    pub fn get_transform_from_angle(&self, player_pos: Vec3, settings: &GameSettings) -> Transform {
        let rad = self.current_angle.to_radians();
        Transform::from_xyz(
            player_pos.x + settings.camera_distance * rad.cos(),
            player_pos.y + settings.camera_height,
            player_pos.z + settings.camera_distance * rad.sin()
        ).looking_at(player_pos, Vec3::Y)
    }
}

#[derive(Component, Debug)]
pub struct CameraRotationController {
    pub rotation_speed: f32,
    pub rotation_timer: Timer,
    pub is_rotating: bool,
    pub snap_start_angle: f32,
    pub continuous_rotation: bool,
    pub rotation_direction: bool,
    pub continuous_rotation_elapsed: f32,
    pub rotation_acceleration_time: f32,
    pub is_decelerating: bool,
    pub deceleration_elapsed: f32,
    pub deceleration_time: f32,
    pub current_velocity: f32,
}

impl CameraRotationController {
    pub fn new(settings: &GameSettings) -> Self {
        Self {
            rotation_speed: settings.camera_rotation_speed,
            rotation_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_rotating: false,
            snap_start_angle: 0.0,
            continuous_rotation: false,
            rotation_direction: false,
            continuous_rotation_elapsed: 0.0,
            rotation_acceleration_time: 0.2,
            is_decelerating: false,
            deceleration_elapsed: 0.0,
            deceleration_time: 0.4,
            current_velocity: 0.0,
        }
    }
}

impl Default for CameraRotationController {
    fn default() -> Self {
        Self {
            rotation_speed: CAMERA_ROTATION_SPEED,
            rotation_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_rotating: false,
            snap_start_angle: 0.0,
            continuous_rotation: false,
            rotation_direction: false,
            continuous_rotation_elapsed: 0.0,
            rotation_acceleration_time: 0.2,
            is_decelerating: false,
            deceleration_elapsed: 0.0,
            deceleration_time: 0.4,
            current_velocity: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct CameraZoomController {
    pub base_zoom: f32,
    pub zoom_out_value: f32,
    pub current_zoom: f32,
    pub is_zooming_out: bool,
    pub is_zooming_in: bool,
    pub zoom_elapsed: f32,
    pub zoom_duration: f32,
}

impl CameraZoomController {
    pub fn new(settings: &GameSettings) -> Self {
        Self {
            base_zoom: settings.base_zoom,
            zoom_out_value: settings.zoom_out_value,
            current_zoom: settings.base_zoom,
            is_zooming_out: false,
            is_zooming_in: false,
            zoom_elapsed: 0.0,
            zoom_duration: 0.3,
        }
    }
}

impl Default for CameraZoomController {
    fn default() -> Self {
        Self {
            base_zoom: BASE_ZOOM,
            zoom_out_value: ZOOM_OUT_VALUE,
            current_zoom: BASE_ZOOM,
            is_zooming_out: false,
            is_zooming_in: false,
            zoom_elapsed: 0.0,
            zoom_duration: 0.3,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CameraPositionController {
    pub current_position: usize,
}

impl Default for CameraPositionController {
    fn default() -> Self {
        Self {
            current_position: 0,
        }
    }
} 