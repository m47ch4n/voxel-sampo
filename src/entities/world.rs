use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_room(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

pub fn spawn_lighting(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
