use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GroundDetection {
    pub velocity_threshold: f32,
    pub height_threshold: f32,
}

impl Default for GroundDetection {
    fn default() -> Self {
        Self {
            velocity_threshold: 0.1,
            height_threshold: 1.1,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct DynamicDamping {
    pub ground_damping: f32,
    pub air_damping: f32,
}

impl Default for DynamicDamping {
    fn default() -> Self {
        Self {
            ground_damping: 8.0,
            air_damping: 0.1,
        }
    }
}
