use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};
use crate::config::{GameSettings, CAMERA_ANGLES};
use crate::player::Player;
use super::components::{CameraAngle, CameraRotationController, CameraZoomController, CameraPositionController};

pub fn camera_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut CameraRotationController, &mut CameraZoomController)>,
) {
    if let Ok((mut rotation_controller, mut zoom_controller)) = camera_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyE) {
            if !rotation_controller.continuous_rotation {
                rotation_controller.continuous_rotation = true;
                rotation_controller.rotation_direction = true;
                rotation_controller.is_rotating = false;
                rotation_controller.continuous_rotation_elapsed = 0.0;
                rotation_controller.is_decelerating = false;
                zoom_controller.is_zooming_out = true;
                zoom_controller.is_zooming_in = false;
                zoom_controller.zoom_elapsed = 0.0;
            }
        }
        else if keyboard_input.pressed(KeyCode::KeyQ) {
            if !rotation_controller.continuous_rotation {
                rotation_controller.continuous_rotation = true;
                rotation_controller.rotation_direction = false;
                rotation_controller.is_rotating = false;
                rotation_controller.continuous_rotation_elapsed = 0.0;
                rotation_controller.is_decelerating = false;
                zoom_controller.is_zooming_out = true;
                zoom_controller.is_zooming_in = false;
                zoom_controller.zoom_elapsed = 0.0;
            }
        }
        else if keyboard_input.just_released(KeyCode::KeyQ) || keyboard_input.just_released(KeyCode::KeyE) {
            if rotation_controller.continuous_rotation {
                rotation_controller.continuous_rotation = false;
                rotation_controller.is_decelerating = true;
                rotation_controller.deceleration_elapsed = 0.0;
                
                let acceleration_progress = (rotation_controller.continuous_rotation_elapsed / rotation_controller.rotation_acceleration_time).min(1.0);
                let speed_multiplier = acceleration_progress * acceleration_progress;
                let min_speed_ratio = 0.1;
                let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
                rotation_controller.current_velocity = rotation_controller.rotation_speed * final_speed_multiplier;
                
                zoom_controller.is_zooming_out = false;
                zoom_controller.is_zooming_in = true;
                zoom_controller.zoom_elapsed = 0.0;
            }
        }
    }
}

pub fn camera_zoom_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraZoomController, &mut Projection), With<Camera3d>>,
) {
    if let Ok((mut zoom_controller, mut projection)) = camera_query.get_single_mut() {
        if zoom_controller.is_zooming_out || zoom_controller.is_zooming_in {
            zoom_controller.zoom_elapsed += time.delta_seconds();
            let zoom_progress = (zoom_controller.zoom_elapsed / zoom_controller.zoom_duration).min(1.0);
            
            if zoom_controller.is_zooming_out {
                let eased_progress = zoom_progress * zoom_progress;
                zoom_controller.current_zoom = zoom_controller.base_zoom + 
                    (zoom_controller.zoom_out_value - zoom_controller.base_zoom) * eased_progress;
                
                if zoom_progress >= 1.0 {
                    zoom_controller.is_zooming_out = false;
                    zoom_controller.current_zoom = zoom_controller.zoom_out_value;
                }
            } else if zoom_controller.is_zooming_in {
                let eased_progress = 1.0 - (1.0 - zoom_progress) * (1.0 - zoom_progress);
                zoom_controller.current_zoom = zoom_controller.zoom_out_value + 
                    (zoom_controller.base_zoom - zoom_controller.zoom_out_value) * eased_progress;
                
                if zoom_progress >= 1.0 {
                    zoom_controller.is_zooming_in = false;
                    zoom_controller.current_zoom = zoom_controller.base_zoom;
                }
            }
            
            if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
                ortho.scaling_mode = ScalingMode::FixedVertical(zoom_controller.current_zoom);
            }
        }
    }
}

pub fn camera_rotation_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraRotationController, &mut CameraPositionController, &mut CameraAngle), With<Camera3d>>,
) {
    if let Ok((mut rotation_controller, mut position_controller, mut camera_angle)) = camera_query.get_single_mut() {
        if rotation_controller.continuous_rotation {
            rotation_controller.continuous_rotation_elapsed += time.delta_seconds();
            
            let acceleration_progress = (rotation_controller.continuous_rotation_elapsed / rotation_controller.rotation_acceleration_time).min(1.0);
            let speed_multiplier = acceleration_progress * acceleration_progress;
            
            let min_speed_ratio = 0.1;
            let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
            
            let current_rotation_speed = rotation_controller.rotation_speed * final_speed_multiplier;
            let rotation_delta = current_rotation_speed * time.delta_seconds();
            
            if rotation_controller.rotation_direction {
                camera_angle.current_angle += rotation_delta;
            } else {
                camera_angle.current_angle -= rotation_delta;
            }
            
            camera_angle.current_angle = camera_angle.current_angle % 360.0;
            if camera_angle.current_angle < 0.0 {
                camera_angle.current_angle += 360.0;
            }
        }
        else if rotation_controller.is_decelerating {
            rotation_controller.deceleration_elapsed += time.delta_seconds();
            
            let deceleration_progress = (rotation_controller.deceleration_elapsed / rotation_controller.deceleration_time).min(1.0);
            let remaining_speed_ratio = 1.0 - (deceleration_progress * deceleration_progress);
            
            let current_speed = rotation_controller.current_velocity * remaining_speed_ratio;
            let rotation_delta = current_speed * time.delta_seconds();
            
            if rotation_controller.rotation_direction {
                camera_angle.current_angle += rotation_delta;
            } else {
                camera_angle.current_angle -= rotation_delta;
            }
            
            camera_angle.current_angle = camera_angle.current_angle % 360.0;
            if camera_angle.current_angle < 0.0 {
                camera_angle.current_angle += 360.0;
            }
            
            if deceleration_progress >= 1.0 {
                rotation_controller.is_decelerating = false;
                
                let angles = CAMERA_ANGLES;
                let current_angle = camera_angle.current_angle;
                let mut closest_index = 0;
                let mut min_distance = f32::MAX;
                
                for (i, &angle) in angles.iter().enumerate() {
                    let distance = (angle - current_angle).abs();
                    let wrapped_distance = (360.0 - distance).min(distance);
                    if wrapped_distance < min_distance {
                        min_distance = wrapped_distance;
                        closest_index = i;
                    }
                }
                
                if min_distance <= 5.0 {
                    position_controller.current_position = closest_index;
                    camera_angle.current_angle = angles[closest_index];
                } else {
                    position_controller.current_position = closest_index;
                    rotation_controller.snap_start_angle = current_angle;
                    rotation_controller.is_rotating = true;
                    rotation_controller.rotation_timer.reset();
                }
            }
        }
        else if rotation_controller.is_rotating {
            rotation_controller.rotation_timer.tick(time.delta());

            let progress = rotation_controller.rotation_timer.elapsed_secs()
                / rotation_controller.rotation_timer.duration().as_secs_f32();

            if progress >= 1.0 {
                rotation_controller.is_rotating = false;
                
                let angles = CAMERA_ANGLES;
                camera_angle.current_angle = angles[position_controller.current_position];
            } else {
                let angles = CAMERA_ANGLES;
                let target_angle = angles[position_controller.current_position];
                
                let start_angle = rotation_controller.snap_start_angle;
                
                let mut angle_diff = target_angle - start_angle;
                
                if angle_diff > 180.0 {
                    angle_diff -= 360.0;
                } else if angle_diff < -180.0 {
                    angle_diff += 360.0;
                }
                
                let eased_progress = if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                };
                
                let current_interpolated_angle = start_angle + angle_diff * eased_progress;
                
                let normalized_angle = if current_interpolated_angle < 0.0 {
                    current_interpolated_angle + 360.0
                } else if current_interpolated_angle >= 360.0 {
                    current_interpolated_angle - 360.0
                } else {
                    current_interpolated_angle
                };
                
                camera_angle.current_angle = normalized_angle;
            }
        }
    }
}

pub fn camera_follow_system(
    mut camera_query: Query<(&CameraAngle, &mut Transform), With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    settings: Res<GameSettings>,
) {
    if let Ok((camera_angle, mut transform)) = camera_query.get_single_mut() {
        let player_pos = if let Ok(player_transform) = player_query.get_single() {
            player_transform.translation
        } else {
            Vec3::ZERO
        };

        *transform = camera_angle.get_transform_from_angle(player_pos, &settings);
    }
} 