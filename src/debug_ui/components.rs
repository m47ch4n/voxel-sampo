use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DebugState {
    pub enabled: bool,
    pub show_physics_debug: bool,
}

#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
pub struct DebugOverlay;
