use super::components::{GroundedState, Player};
use crate::camera::CameraAngle;
use crate::config::Config;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut ExternalForce, &GroundedState), With<Player>>,
    camera_query: Query<&CameraAngle>,
    config: Res<Config>,
) {
    if let Ok((player, mut external_force, grounded_state)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if let Ok(camera_angle) = camera_query.single() {
            let forward = camera_angle.get_camera_forward_direction();
            let right = camera_angle.get_camera_right_direction();

            if keyboard_input.pressed(config.key_bindings.player_move_up) {
                direction += forward;
            }
            if keyboard_input.pressed(config.key_bindings.player_move_down) {
                direction -= forward;
            }
            if keyboard_input.pressed(config.key_bindings.player_move_left) {
                direction -= right;
            }
            if keyboard_input.pressed(config.key_bindings.player_move_right) {
                direction += right;
            }
        }

        direction.y = 0.0;

        let mut force = Vec3::ZERO;

        if direction.length() > 0.0 {
            if direction.x.abs() > direction.z.abs() {
                direction = Vec3::new(direction.x.signum(), 0.0, 0.0);
            } else {
                direction = Vec3::new(0.0, 0.0, direction.z.signum());
            }

            if grounded_state.is_grounded {
                force += direction * player.move_force;
            } else {
                force += direction * player.air_control_force;
            }
        }

        if keyboard_input.just_pressed(config.key_bindings.player_jump)
            && grounded_state.is_grounded
        {
            force.y += player.jump_force;
        }

        external_force.force = force;
    }
}

pub fn player_velocity_limit_system(
    mut player_query: Query<(&Player, &mut Velocity), With<Player>>,
) {
    if let Ok((player, mut velocity)) = player_query.single_mut() {
        let mut horizontal_velocity = Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z);
        if horizontal_velocity.length() > player.max_speed {
            horizontal_velocity = horizontal_velocity.normalize() * player.max_speed;
            velocity.linvel.x = horizontal_velocity.x;
            velocity.linvel.z = horizontal_velocity.z;
        }
    }
}
