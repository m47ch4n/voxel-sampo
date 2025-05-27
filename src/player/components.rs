use bevy::prelude::*;
use crate::config::PLAYER_MOVE_DURATION;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub move_timer: Timer,
    pub target_position: Vec3,
    pub is_moving: bool,
}

impl Player {
    // pub fn new(settings: &GameSettings) -> Self {
    //     Self {
    //         move_timer: Timer::from_seconds(settings.player_move_duration, TimerMode::Once),
    //         target_position: Vec3::ZERO,
    //         is_moving: false,
    //     }
    // }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            move_timer: Timer::from_seconds(PLAYER_MOVE_DURATION, TimerMode::Once),
            target_position: Vec3::ZERO,
            is_moving: false,
        }
    }
} 