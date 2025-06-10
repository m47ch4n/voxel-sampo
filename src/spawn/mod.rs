use bevy::{
    core_pipeline::{
        bloom::Bloom, core_3d::ScreenSpaceTransmissionQuality, tonemapping::Tonemapping,
    },
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_rapier3d::prelude::*;

use crate::camera::{
    CameraAngle, CameraPositionController, CameraRotationController, CameraZoomController,
};
use crate::config::Config;
use crate::player::{GroundedState, Player};

pub fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<Config>,
) {
    spawn_player(&mut commands, &mut meshes, &mut materials, &config);
    spawn_camera(&mut commands, &config);
    spawn_lighting(&mut commands, &asset_server);
    spawn_room(&mut commands, &asset_server);
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &Res<Config>,
) {
    let initial_player_pos = Vec3::new(0.0, 1.0, 0.0);
    commands.spawn((
        Player::new_with_config(&config.player),
        GroundedState { is_grounded: false },
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.9),
            metallic: 0.1,
            perceptual_roughness: 0.3,
            reflectance: 0.5,
            ..default()
        })),
        Transform::from_xyz(
            initial_player_pos.x,
            initial_player_pos.y,
            initial_player_pos.z,
        ),
        RigidBody::Dynamic,
        Collider::cuboid(0.25, 0.25, 0.25),
        ExternalForce::default(),
        Velocity::default(),
        Restitution::coefficient(0.0),
        Friction::coefficient(1.5),
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
    ));
}

fn spawn_camera(commands: &mut Commands, config: &Res<Config>) {
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

fn spawn_room(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("room.vox")),
        Transform::from_scale(Vec3::splat(0.05)),
        RigidBody::Fixed,
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::empty())),
            ..default()
        },
    ));
}

fn spawn_lighting(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(EnvironmentMapLight {
        diffuse_map: asset_server.load("papermill_diffuse.ktx2"),
        specular_map: asset_server.load("papermill_specular.ktx2"),
        intensity: 300.0,
        rotation: Quat::IDENTITY,
        affects_lightmapped_mesh_diffuse: true,
    });

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.99, 1.0),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::IDENTITY.looking_to(Vec3::new(1.0, -2.5, 0.85), Vec3::Y),
    ));
}
