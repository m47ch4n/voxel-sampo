use bevy::{
    core_pipeline::{
        bloom::BloomSettings, core_3d::ScreenSpaceTransmissionQuality, tonemapping::Tonemapping,
    },
    prelude::*,
    render::camera::ScalingMode,
    window::WindowResolution,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle};

fn main() {
    let mut app = App::new();

    let custom_default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Voxel Sampo".to_string(),
            resolution: WindowResolution::new(960., 720.).with_scale_factor_override(1.),
            resizable: false,
            ..default()
        }),
        ..default()
    });

    app.add_plugins((custom_default_plugins, PanOrbitCameraPlugin, VoxScenePlugin))
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(10.0),
                ..default()
            }
            .into(),
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::High,
                screen_space_specular_transmission_steps: 1,
                ..default()
            },
            transform: Transform::from_xyz(-8.0, 8.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
            ..Default::default()
        },
        PanOrbitCamera::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: assets.load("papermill_diffuse.ktx2"),
            specular_map: assets.load("papermill_specular.ktx2"),
            intensity: 300.0,
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(1.0, -2.5, 0.85), Vec3::Y),
        ..default()
    });

    commands.spawn(VoxelSceneBundle {
        scene: assets.load("room.vox"),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..default()
    });
}
