use bevy::prelude::*;
use crate::config::PlayerConfig;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub move_timer: Timer,
    pub target_position: Vec3,
    pub is_moving: bool,
}

impl Player {
    pub fn new_with_config(player_config: &PlayerConfig) -> Self {
        Self {
            move_timer: Timer::from_seconds(player_config.move_duration, TimerMode::Once),
            target_position: Vec3::new(0.0, 0.5, 0.0),
            is_moving: false,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let default_config = PlayerConfig::default();
        Self {
            move_timer: Timer::from_seconds(default_config.move_duration, TimerMode::Once),
            target_position: Vec3::new(0.0, 0.5, 0.0),
            is_moving: false,
        }
    }
} 