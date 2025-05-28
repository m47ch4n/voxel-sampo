use super::components::{
    CameraAngle, CameraPositionController, CameraRotationController, CameraZoomController,
    RotationDirection, RotationState,
};
use crate::config::{Config, CAMERA_ANGLES};
use crate::player::Player;
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn camera_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<Config>,
    mut camera_query: Query<&mut CameraRotationController>,
) {
    if let Ok(mut rotation_controller) = camera_query.single_mut() {
        let cw_pressed = keyboard_input.pressed(config.key_bindings.camera_rotate_clockwise);
        let ccw_pressed =
            keyboard_input.pressed(config.key_bindings.camera_rotate_counter_clockwise);

        rotation_controller.set_throttle(cw_pressed, ccw_pressed);
    }
}

pub fn camera_zoom_system(
    time: Res<Time>,
    mut camera_query: Query<
        (
            &CameraRotationController,
            &mut CameraZoomController,
            &mut Projection,
        ),
        With<Camera3d>,
    >,
) {
    if let Ok((rotation_controller, mut zoom_controller, mut projection)) =
        camera_query.single_mut()
    {
        let current_velocity = rotation_controller.get_current_velocity().abs();
        zoom_controller.update_zoom_for_velocity(
            current_velocity,
            rotation_controller.max_velocity,
            time.delta_secs(),
        );

        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: zoom_controller.current_zoom,
            };
        }
    }
}

pub fn camera_rotation_system(
    time: Res<Time>,
    mut camera_query: Query<
        (
            &mut CameraRotationController,
            &mut CameraPositionController,
            &mut CameraAngle,
        ),
        With<Camera3d>,
    >,
) {
    if let Ok((mut rotation_controller, mut position_controller, mut camera_angle)) =
        camera_query.single_mut()
    {
        match rotation_controller.state.clone() {
            RotationState::Idle => {
                if let Some(direction) = rotation_controller.get_rotation_direction() {
                    let current_index = position_controller.current_snap_index;
                    let target_index = match direction {
                        RotationDirection::Clockwise => (current_index + 1) % CAMERA_ANGLES.len(),
                        RotationDirection::CounterClockwise => {
                            if current_index == 0 {
                                CAMERA_ANGLES.len() - 1
                            } else {
                                current_index - 1
                            }
                        }
                    };

                    rotation_controller.state = RotationState::Rotating {
                        direction,
                        current_velocity: 0.0,
                        target_snap_index: target_index,
                        momentum: 0.0,
                    };
                }
            }
            RotationState::Rotating {
                direction,
                current_velocity,
                target_snap_index,
                momentum,
            } => {
                let mut new_velocity = current_velocity;
                let mut new_target = target_snap_index;
                let mut new_momentum = momentum;

                match rotation_controller.get_rotation_direction() {
                    Some(input_direction) if input_direction == direction => {
                        new_velocity = (new_velocity
                            + rotation_controller.acceleration * time.delta_secs())
                        .min(rotation_controller.max_velocity);
                        new_momentum = new_velocity;
                    }
                    Some(_) => {
                        rotation_controller.state = RotationState::Idle;
                        return;
                    }
                    None => {
                        new_velocity = (new_velocity
                            - rotation_controller.deceleration * time.delta_secs())
                        .max(0.0);
                    }
                }

                let rotation_delta = new_velocity * time.delta_secs();
                match direction {
                    RotationDirection::Clockwise => {
                        camera_angle.current_angle += rotation_delta;
                    }
                    RotationDirection::CounterClockwise => {
                        camera_angle.current_angle -= rotation_delta;
                    }
                }

                camera_angle.current_angle = camera_angle.current_angle % 360.0;
                if camera_angle.current_angle < 0.0 {
                    camera_angle.current_angle += 360.0;
                }

                let target_angle = CAMERA_ANGLES[new_target];
                let angle_diff = match direction {
                    RotationDirection::Clockwise => {
                        let diff = target_angle - camera_angle.current_angle;
                        if diff < 0.0 {
                            diff + 360.0
                        } else {
                            diff
                        }
                    }
                    RotationDirection::CounterClockwise => {
                        let diff = camera_angle.current_angle - target_angle;
                        if diff < 0.0 {
                            diff + 360.0
                        } else {
                            diff
                        }
                    }
                };

                if angle_diff <= 5.0 && new_velocity <= rotation_controller.momentum_threshold {
                    camera_angle.current_angle = target_angle;
                    position_controller.current_snap_index = new_target;
                    rotation_controller.state = RotationState::Idle;
                } else if angle_diff <= 5.0 && new_velocity > rotation_controller.momentum_threshold
                {
                    let next_target = match direction {
                        RotationDirection::Clockwise => (new_target + 1) % CAMERA_ANGLES.len(),
                        RotationDirection::CounterClockwise => {
                            if new_target == 0 {
                                CAMERA_ANGLES.len() - 1
                            } else {
                                new_target - 1
                            }
                        }
                    };

                    position_controller.current_snap_index = new_target;
                    new_target = next_target;

                    new_velocity *= 0.8;
                }

                if new_velocity <= 0.01 {
                    rotation_controller.state = RotationState::Idle;
                } else {
                    rotation_controller.state = RotationState::Rotating {
                        direction,
                        current_velocity: new_velocity,
                        target_snap_index: new_target,
                        momentum: new_momentum,
                    };
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
