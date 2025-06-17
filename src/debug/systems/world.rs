use crate::camera::CameraAngle;
use bevy::prelude::*;

pub struct WorldInfo {
    pub facing_direction: String,
}

impl WorldInfo {
    pub fn from_camera_query(camera_query: &Query<&CameraAngle>) -> Self {
        let facing_direction = if let Ok(camera_angle) = camera_query.single() {
            let forward = camera_angle.get_camera_forward_direction();
            if forward.z < -0.5 {
                "North (-Z)".to_string()
            } else if forward.z > 0.5 {
                "South (+Z)".to_string()
            } else if forward.x > 0.5 {
                "East (+X)".to_string()
            } else if forward.x < -0.5 {
                "West (-X)".to_string()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        Self { facing_direction }
    }

    pub fn format(&self) -> String {
        format!("Facing: {}", self.facing_direction)
    }
}
