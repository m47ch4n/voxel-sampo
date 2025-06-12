use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GroundDetection {
    pub ray_distance: f32,
}

impl Default for GroundDetection {
    fn default() -> Self {
        Self {
            ray_distance: 0.6,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct DynamicDamping {
    pub ground_damping: f32,
    pub air_damping: f32,
}

#[derive(Component, Debug, Clone)]
pub struct GroundRay {
    pub origin: Vec3,
    pub direction: Vec3,
    pub distance: f32,
    pub hit_point: Option<Vec3>,
}

impl Default for GroundRay {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::NEG_Y,
            distance: 0.2,
            hit_point: None,
        }
    }
}

impl Default for DynamicDamping {
    fn default() -> Self {
        Self {
            ground_damping: 8.0,
            air_damping: 0.1,
        }
    }
}
