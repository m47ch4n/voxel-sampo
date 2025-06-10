use super::systems::{
    dynamic_damping_system, ground_detection_system, player_input_system,
    player_rotation_lock_system, player_velocity_limit_system,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            ground_detection_system,
            dynamic_damping_system,
            player_input_system,
            player_velocity_limit_system,
            player_rotation_lock_system,
        ),
    );
}
