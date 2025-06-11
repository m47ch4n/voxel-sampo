use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::{DynamicDamping, GroundDetection};
use crate::player::{GroundedState, Player};

pub fn ground_detection_system(
    mut query: Query<(&mut GroundedState, &Transform, &Velocity, &GroundDetection), With<Player>>,
) {
    for (mut grounded_state, transform, velocity, ground_detection) in query.iter_mut() {
        grounded_state.is_grounded = velocity.linvel.y.abs() < ground_detection.velocity_threshold
            && transform.translation.y <= ground_detection.height_threshold;
    }
}

pub fn dynamic_damping_system(
    mut query: Query<(&GroundedState, &mut Damping, &DynamicDamping), With<Player>>,
) {
    for (grounded_state, mut damping, dynamic_damping) in query.iter_mut() {
        if grounded_state.is_grounded {
            damping.linear_damping = dynamic_damping.ground_damping;
        } else {
            damping.linear_damping = dynamic_damping.air_damping;
        }
    }
}

pub fn player_rotation_lock_system(mut player_query: Query<&mut Transform, With<Player>>) {
    for mut transform in player_query.iter_mut() {
        transform.rotation = Quat::IDENTITY;
    }
}
