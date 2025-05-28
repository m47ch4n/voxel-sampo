use crate::config::{CameraConfig, BASE_ZOOM, CAMERA_ANGLES, ZOOM_OUT_VALUE};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Component, Debug, Clone)]
pub struct CameraAngle {
    pub current_angle: f32,
}

impl Default for CameraAngle {
    fn default() -> Self {
        Self {
            current_angle: CAMERA_ANGLES[3],
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

    pub fn get_transform_from_angle(
        &self,
        player_pos: Vec3,
        camera_config: &CameraConfig,
    ) -> Transform {
        let rad = self.current_angle.to_radians();
        Transform::from_xyz(
            player_pos.x + camera_config.distance * rad.cos(),
            player_pos.y + camera_config.height,
            player_pos.z + camera_config.distance * rad.sin(),
        )
        .looking_at(player_pos, Vec3::Y)
    }
}

#[derive(Debug, Clone)]
pub enum RotationState {
    Idle,
    Rotating {
        direction: RotationDirection,
        current_velocity: f32,
        target_snap_index: usize,
        momentum: f32, // accumulated rotation momentum
    },
}

#[derive(Component, Debug)]
pub struct CameraRotationController {
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_velocity: f32,
    pub momentum_threshold: f32,
    pub state: RotationState,
    pub throttle_cw: bool,  // clockwise throttle
    pub throttle_ccw: bool, // counter-clockwise throttle
}

impl CameraRotationController {
    pub fn new(_camera_config: &CameraConfig) -> Self {
        Self {
            acceleration: 720.0,  // degrees/second^2
            deceleration: 1440.0, // degrees/second^2
            max_velocity: 540.0,  // degrees/second
            momentum_threshold: 270.0,
            state: RotationState::Idle,
            throttle_cw: false,
            throttle_ccw: false,
        }
    }

    pub fn set_throttle(&mut self, cw: bool, ccw: bool) {
        self.throttle_cw = cw;
        self.throttle_ccw = ccw;
    }

    pub fn get_rotation_direction(&self) -> Option<RotationDirection> {
        match (self.throttle_cw, self.throttle_ccw) {
            (true, false) => Some(RotationDirection::Clockwise),
            (false, true) => Some(RotationDirection::CounterClockwise),
            _ => None, // both or neither pressed
        }
    }

    pub fn get_current_velocity(&self) -> f32 {
        match &self.state {
            RotationState::Rotating {
                current_velocity, ..
            } => *current_velocity,
            _ => 0.0,
        }
    }
}

impl Default for CameraRotationController {
    fn default() -> Self {
        Self {
            acceleration: 720.0,
            deceleration: 1440.0,
            max_velocity: 540.0,
            momentum_threshold: 270.0,
            state: RotationState::Idle,
            throttle_cw: false,
            throttle_ccw: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct CameraZoomController {
    pub base_zoom: f32,
    pub max_zoom_out: f32,
    pub current_zoom: f32,
    pub zoom_speed: f32,
}

impl CameraZoomController {
    pub fn new(camera_config: &CameraConfig) -> Self {
        Self {
            base_zoom: camera_config.base_zoom,
            max_zoom_out: camera_config.zoom_out_value,
            current_zoom: camera_config.base_zoom,
            zoom_speed: 20.0,
        }
    }

    pub fn update_zoom_for_velocity(&mut self, velocity: f32, max_velocity: f32, time_delta: f32) {
        let velocity_ratio = (velocity / max_velocity).min(1.0);
        let target_zoom = self.base_zoom + (self.max_zoom_out - self.base_zoom) * velocity_ratio;

        let zoom_diff = target_zoom - self.current_zoom;
        let zoom_change = zoom_diff * self.zoom_speed * time_delta;
        self.current_zoom += zoom_change;
    }
}

impl Default for CameraZoomController {
    fn default() -> Self {
        Self {
            base_zoom: BASE_ZOOM,
            max_zoom_out: ZOOM_OUT_VALUE,
            current_zoom: BASE_ZOOM,
            zoom_speed: 20.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CameraPositionController {
    pub current_snap_index: usize,
}

impl Default for CameraPositionController {
    fn default() -> Self {
        Self {
            current_snap_index: 0,
        }
    }
}
