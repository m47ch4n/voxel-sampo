use bevy::{
    core_pipeline::{
        bloom::Bloom, core_3d::ScreenSpaceTransmissionQuality, tonemapping::Tonemapping,
    },
    prelude::*,
    render::camera::ScalingMode,
};

use crate::camera::{
    CameraAngle, CameraPositionController, CameraRotationController, CameraZoomController,
};
use crate::config::Config;

pub fn spawn_camera(commands: &mut Commands, config: &Res<Config>) {
    let rotation_controller = CameraRotationController::new(&config.camera);
    let zoom_controller = CameraZoomController::new(&config.camera);
    let camera_angle = CameraAngle::default();

    let initial_transform = camera_angle.get_transform_from_angle(Vec3::ZERO, &config.camera);

    commands.spawn((
        Camera3d {
            screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::High,
            screen_space_specular_transmission_steps: 1,
            ..default()
        },
        Camera {
            hdr: true,
            ..Default::default()
        },
        initial_transform,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.camera.base_zoom,
            },
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        Tonemapping::SomewhatBoringDisplayTransform,
        Bloom {
            intensity: 0.3,
            ..default()
        },
        camera_angle,
        rotation_controller,
        zoom_controller,
        CameraPositionController::default(),
    ));
}
