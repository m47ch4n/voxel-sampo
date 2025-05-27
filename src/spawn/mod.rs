use bevy::{
    core_pipeline::{
        bloom::BloomSettings, core_3d::ScreenSpaceTransmissionQuality, tonemapping::Tonemapping,
    },
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_vox_scene::VoxelSceneBundle;

use crate::config::GameSettings;
use crate::player::Player;
use crate::camera::{CameraAngle, CameraRotationController, CameraZoomController, CameraPositionController};

pub fn spawn_entities(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<GameSettings>,
) {
    spawn_camera(&mut commands, &assets, &settings);
    spawn_lighting(&mut commands);
    spawn_world(&mut commands, &assets);
    spawn_player(&mut commands, &mut meshes, &mut materials, &settings);
}

fn spawn_camera(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    settings: &Res<GameSettings>,
) {
    let camera_position_controller = CameraPositionController::default();
    let camera_rotation_controller = CameraRotationController::new(settings);
    let camera_zoom_controller = CameraZoomController::new(settings);
    let camera_angle = CameraAngle::default();
    let initial_transform = camera_angle.get_transform_from_angle(Vec3::ZERO, settings);

    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(settings.base_zoom),
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
            transform: initial_transform,
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
            ..Default::default()
        },
        PanOrbitCamera::default(),
        camera_position_controller,
        camera_rotation_controller,
        camera_zoom_controller,
        camera_angle,
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
}

fn spawn_lighting(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(1.0, -2.5, 0.85), Vec3::Y),
        ..default()
    });
}

fn spawn_world(commands: &mut Commands, assets: &Res<AssetServer>) {
    commands.spawn(VoxelSceneBundle {
        scene: assets.load("room.vox"),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..default()
    });
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    settings: &Res<GameSettings>,
) {
    let initial_player_pos = Vec3::new(0.0, 0.8, 0.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.8, 1.6, 0.8)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                perceptual_roughness: 0.8,
                metallic: 0.1,
                ..default()
            }),
            transform: Transform::from_xyz(initial_player_pos.x, initial_player_pos.y, initial_player_pos.z),
            ..default()
        },
        Player {
            move_timer: Timer::from_seconds(settings.player_move_duration, TimerMode::Once),
            target_position: initial_player_pos,
            is_moving: false,
        },
    ));
} 