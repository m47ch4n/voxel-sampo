use super::components::Player;
use crate::camera::CameraAngle;
use crate::config::Config;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut ExternalForce), With<Player>>,
    camera_query: Query<&CameraAngle>,
    config: Res<Config>,
) {
    if let Ok((player, mut external_force)) = player_query.single_mut() {
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

        // 水平方向の移動のみ（Y軸は0にする）
        direction.y = 0.0;
        
        // 元の4方向移動システムを再現：東西南北のいずれかに制限
        if direction.length() > 0.0 {
            if direction.x.abs() > direction.z.abs() {
                // X軸方向が優勢：東西移動
                direction = Vec3::new(direction.x.signum(), 0.0, 0.0);
            } else {
                // Z軸方向が優勢：南北移動
                direction = Vec3::new(0.0, 0.0, direction.z.signum());
            }
            external_force.force = direction * player.move_force;
        } else {
            external_force.force = Vec3::ZERO;
        }
    }
}

pub fn player_velocity_limit_system(
    mut player_query: Query<(&Player, &mut Velocity), With<Player>>,
) {
    if let Ok((player, mut velocity)) = player_query.single_mut() {
        // 水平方向の速度制限
        let mut horizontal_velocity = Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z);
        if horizontal_velocity.length() > player.max_speed {
            horizontal_velocity = horizontal_velocity.normalize() * player.max_speed;
            velocity.linvel.x = horizontal_velocity.x;
            velocity.linvel.z = horizontal_velocity.z;
        }
    }
}
