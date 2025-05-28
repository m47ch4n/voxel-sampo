use bevy::prelude::*;
use crate::config::{CameraConfig, CAMERA_ANGLES, CAMERA_ROTATION_SPEED, BASE_ZOOM, ZOOM_OUT_VALUE};

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

    pub fn get_transform_from_angle(&self, player_pos: Vec3, camera_config: &CameraConfig) -> Transform {
        let rad = self.current_angle.to_radians();
        Transform::from_xyz(
            player_pos.x + camera_config.distance * rad.cos(),
            player_pos.y + camera_config.height,
            player_pos.z + camera_config.distance * rad.sin()
        ).looking_at(player_pos, Vec3::Y)
    }
}

#[derive(Debug, Clone)]
pub enum RotationState {
    Idle,
    Continuous {
        direction: bool,
        elapsed: f32,
        acceleration_time: f32,
    },
    Decelerating {
        direction: bool,
        elapsed: f32,
        deceleration_time: f32,
        initial_velocity: f32,
    },
    Snapping {
        timer: Timer,
        start_angle: f32,
    },
}

#[derive(Component, Debug)]
pub struct CameraRotationController {
    pub rotation_speed: f32,
    pub state: RotationState,
}

impl CameraRotationController {
    pub fn new(camera_config: &CameraConfig) -> Self {
        Self {
            rotation_speed: camera_config.rotation_speed,
            state: RotationState::Idle,
        }
    }

    pub fn start_continuous_rotation(&mut self, clockwise: bool) {
        if matches!(self.state, RotationState::Idle) {
            self.state = RotationState::Continuous {
                direction: clockwise,
                elapsed: 0.0,
                acceleration_time: 0.2,
            };
        }
    }

    pub fn stop_continuous_rotation(&mut self) {
        if let RotationState::Continuous { direction, elapsed, acceleration_time, .. } = self.state {
            let acceleration_progress = (elapsed / acceleration_time).min(1.0);
            let speed_multiplier = acceleration_progress * acceleration_progress;
            let min_speed_ratio = 0.1;
            let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
            let initial_velocity = self.rotation_speed * final_speed_multiplier;

            self.state = RotationState::Decelerating {
                direction,
                elapsed: 0.0,
                deceleration_time: 0.4,
                initial_velocity,
            };
        }
    }

    pub fn start_snap_rotation(&mut self, start_angle: f32) {
        self.state = RotationState::Snapping {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            start_angle,
        };
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, RotationState::Idle)
    }
}

impl Default for CameraRotationController {
    fn default() -> Self {
        Self {
            rotation_speed: CAMERA_ROTATION_SPEED,
            state: RotationState::Idle,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ZoomState {
    Idle,
    ZoomingOut {
        elapsed: f32,
        duration: f32,
        start_zoom: f32,
        target_zoom: f32,
    },
    ZoomingIn {
        elapsed: f32,
        duration: f32,
        start_zoom: f32,
        target_zoom: f32,
    },
}

#[derive(Component, Debug)]
pub struct CameraZoomController {
    pub base_zoom: f32,
    pub zoom_out_value: f32,
    pub current_zoom: f32,
    pub state: ZoomState,
}

impl CameraZoomController {
    pub fn new(camera_config: &CameraConfig) -> Self {
        Self {
            base_zoom: camera_config.base_zoom,
            zoom_out_value: camera_config.zoom_out_value,
            current_zoom: camera_config.base_zoom,
            state: ZoomState::Idle,
        }
    }

    pub fn start_zoom_out(&mut self) {
        if matches!(self.state, ZoomState::Idle) {
            self.state = ZoomState::ZoomingOut {
                elapsed: 0.0,
                duration: 0.3,
                start_zoom: self.current_zoom,
                target_zoom: self.zoom_out_value,
            };
        }
    }

    pub fn start_zoom_in(&mut self) {
        self.state = ZoomState::ZoomingIn {
            elapsed: 0.0,
            duration: 0.3,
            start_zoom: self.current_zoom,
            target_zoom: self.base_zoom,
        };
    }
}

impl Default for CameraZoomController {
    fn default() -> Self {
        Self {
            base_zoom: BASE_ZOOM,
            zoom_out_value: ZOOM_OUT_VALUE,
            current_zoom: BASE_ZOOM,
            state: ZoomState::Idle,
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