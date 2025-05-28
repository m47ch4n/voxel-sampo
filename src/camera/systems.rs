use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};
use crate::config::{Config, CAMERA_ANGLES};
use crate::player::Player;
use super::components::{CameraAngle, CameraRotationController, CameraZoomController, CameraPositionController, RotationState, ZoomState};

pub fn camera_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<Config>,
    mut camera_query: Query<(&mut CameraRotationController, &mut CameraZoomController)>,
) {
    if let Ok((mut rotation_controller, mut zoom_controller)) = camera_query.single_mut() {
        if keyboard_input.pressed(config.key_bindings.camera_rotate_clockwise) {
            if rotation_controller.is_idle() {
                rotation_controller.start_continuous_rotation(true);
                zoom_controller.start_zoom_out();
            }
        }
        else if keyboard_input.pressed(config.key_bindings.camera_rotate_counter_clockwise) {
            if rotation_controller.is_idle() {
                rotation_controller.start_continuous_rotation(false);
                zoom_controller.start_zoom_out();
            }
        }
        else if keyboard_input.just_released(config.key_bindings.camera_rotate_counter_clockwise) || keyboard_input.just_released(config.key_bindings.camera_rotate_clockwise) {
            rotation_controller.stop_continuous_rotation();
            zoom_controller.start_zoom_in();
        }
    }
}

pub fn camera_zoom_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraZoomController, &mut Projection), With<Camera3d>>,
) {
    if let Ok((mut zoom_controller, mut projection)) = camera_query.single_mut() {
        match zoom_controller.state.clone() {
            ZoomState::Idle => {
                // アイドル状態では何もしない
            }
            ZoomState::ZoomingOut { elapsed, duration, start_zoom, target_zoom } => {
                let new_elapsed = elapsed + time.delta_secs();
                let zoom_progress = (new_elapsed / duration).min(1.0);
                
                let eased_progress = zoom_progress * zoom_progress;
                zoom_controller.current_zoom = start_zoom + (target_zoom - start_zoom) * eased_progress;
                
                if zoom_progress >= 1.0 {
                    zoom_controller.current_zoom = target_zoom;
                    zoom_controller.state = ZoomState::Idle;
                } else {
                    zoom_controller.state = ZoomState::ZoomingOut { 
                        elapsed: new_elapsed, 
                        duration, 
                        start_zoom, 
                        target_zoom 
                    };
                }
            }
            ZoomState::ZoomingIn { elapsed, duration, start_zoom, target_zoom } => {
                let new_elapsed = elapsed + time.delta_secs();
                let zoom_progress = (new_elapsed / duration).min(1.0);
                
                let eased_progress = 1.0 - (1.0 - zoom_progress) * (1.0 - zoom_progress);
                zoom_controller.current_zoom = start_zoom + (target_zoom - start_zoom) * eased_progress;
                
                if zoom_progress >= 1.0 {
                    zoom_controller.current_zoom = target_zoom;
                    zoom_controller.state = ZoomState::Idle;
                } else {
                    zoom_controller.state = ZoomState::ZoomingIn { 
                        elapsed: new_elapsed, 
                        duration, 
                        start_zoom, 
                        target_zoom 
                    };
                }
            }
        }
        
        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical { viewport_height: zoom_controller.current_zoom };
        }
    }
}

pub fn camera_rotation_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraRotationController, &mut CameraPositionController, &mut CameraAngle), With<Camera3d>>,
) {
    if let Ok((mut rotation_controller, mut position_controller, mut camera_angle)) = camera_query.single_mut() {
        match rotation_controller.state.clone() {
            RotationState::Idle => {
                // アイドル状態では何もしない
            }
            RotationState::Continuous { direction, elapsed, acceleration_time } => {
                let new_elapsed = elapsed + time.delta_secs();
                
                let acceleration_progress = (new_elapsed / acceleration_time).min(1.0);
                let speed_multiplier = acceleration_progress * acceleration_progress;
                let min_speed_ratio = 0.1;
                let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
                
                let current_rotation_speed = rotation_controller.rotation_speed * final_speed_multiplier;
                let rotation_delta = current_rotation_speed * time.delta_secs();
                
                if direction {
                    camera_angle.current_angle += rotation_delta;
                } else {
                    camera_angle.current_angle -= rotation_delta;
                }
                
                camera_angle.current_angle = camera_angle.current_angle % 360.0;
                if camera_angle.current_angle < 0.0 {
                    camera_angle.current_angle += 360.0;
                }
                
                // 状態を更新
                rotation_controller.state = RotationState::Continuous { 
                    direction, 
                    elapsed: new_elapsed, 
                    acceleration_time 
                };
            }
            RotationState::Decelerating { direction, elapsed, deceleration_time, initial_velocity } => {
                let new_elapsed = elapsed + time.delta_secs();
                
                let deceleration_progress = (new_elapsed / deceleration_time).min(1.0);
                let remaining_speed_ratio = 1.0 - (deceleration_progress * deceleration_progress);
                
                let current_speed = initial_velocity * remaining_speed_ratio;
                let rotation_delta = current_speed * time.delta_secs();
                
                if direction {
                    camera_angle.current_angle += rotation_delta;
                } else {
                    camera_angle.current_angle -= rotation_delta;
                }
                
                camera_angle.current_angle = camera_angle.current_angle % 360.0;
                if camera_angle.current_angle < 0.0 {
                    camera_angle.current_angle += 360.0;
                }
                
                if deceleration_progress >= 1.0 {
                    // 減速終了後、最も近い角度に移動
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
                        rotation_controller.state = RotationState::Idle;
                    } else {
                        position_controller.current_position = closest_index;
                        rotation_controller.start_snap_rotation(current_angle);
                    }
                } else {
                    // 状態を更新
                    rotation_controller.state = RotationState::Decelerating { 
                        direction, 
                        elapsed: new_elapsed, 
                        deceleration_time, 
                        initial_velocity 
                    };
                }
            }
            RotationState::Snapping { mut timer, start_angle } => {
                timer.tick(time.delta());
                
                let progress = timer.elapsed_secs() / timer.duration().as_secs_f32();
                
                if progress >= 1.0 {
                    // スナップ完了
                    let angles = CAMERA_ANGLES;
                    camera_angle.current_angle = angles[position_controller.current_position];
                    rotation_controller.state = RotationState::Idle;
                } else {
                    let angles = CAMERA_ANGLES;
                    let target_angle = angles[position_controller.current_position];
                    
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
                    
                    // 状態を更新
                    rotation_controller.state = RotationState::Snapping { timer, start_angle };
                }
            }
        }
    }
}

pub fn camera_follow_system(
    mut camera_query: Query<(&CameraAngle, &mut Transform), With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    config: Res<Config>,
) {
    if let Ok((camera_angle, mut transform)) = camera_query.single_mut() {
        let player_pos = if let Ok(player_transform) = player_query.single() {
            player_transform.translation
        } else {
            Vec3::ZERO
        };

        *transform = camera_angle.get_transform_from_angle(player_pos, &config.camera);
    }
} 