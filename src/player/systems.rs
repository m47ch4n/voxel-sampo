use super::components::Player;
use crate::camera::CameraAngle;
use crate::config::Config;
use bevy::prelude::*;

pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    camera_query: Query<&CameraAngle>,
    config: Res<Config>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if player.is_moving {
            return;
        }

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

        if direction.length() > 0.0 {
            if direction.x.abs() > direction.z.abs() {
                direction = Vec3::new(direction.x.signum(), 0.0, 0.0);
            } else {
                direction = Vec3::new(0.0, 0.0, direction.z.signum());
            }

            player.target_position += direction * config.player.move_speed;
            player.is_moving = true;
            player.move_timer.reset();
        }
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    if let Ok((mut player, mut transform)) = player_query.single_mut() {
        if player.is_moving {
            player.move_timer.tick(time.delta());

            let start_pos = transform.translation;
            let progress =
                player.move_timer.elapsed_secs() / player.move_timer.duration().as_secs_f32();

            if progress >= 1.0 {
                transform.translation = player.target_position;
                player.is_moving = false;
            } else {
                let eased_progress = if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                };
                transform.translation = start_pos.lerp(player.target_position, eased_progress);
            }
        }
    }
}
