use crate::config::PlayerConfig;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub move_force: f32,
    pub max_speed: f32,
    pub jump_force: f32,
    pub air_control_force: f32,
}

#[derive(Component, Debug, Clone)]
pub struct GroundedState {
    pub is_grounded: bool,
}

impl Player {
    pub fn new_with_config(player_config: &PlayerConfig) -> Self {
        Self {
            move_force: player_config.move_force,
            max_speed: player_config.max_speed,
            jump_force: player_config.jump_force,
            air_control_force: player_config.air_control_force,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let default_config = PlayerConfig::default();
        Self {
            move_force: default_config.move_force,
            max_speed: default_config.max_speed,
            jump_force: default_config.jump_force,
            air_control_force: default_config.air_control_force,
        }
    }
}
