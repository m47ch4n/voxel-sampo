use bevy::prelude::*;
use super::systems::{player_input_system, player_movement_system};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input_system,
            player_movement_system.after(player_input_system),
        ),
    );
} 