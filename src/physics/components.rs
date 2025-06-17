use bevy::prelude::*;

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
