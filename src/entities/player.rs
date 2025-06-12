use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::bundles::PlayerBundle;
use crate::config::Config;
use crate::physics::{DynamicDamping, GroundDetection, GroundRay};
use crate::player::{GroundedState, Player};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &Res<Config>,
) {
    let initial_player_pos = Vec3::new(0.0, 1.0, 0.0);
    commands.spawn(PlayerBundle {
        player: Player::new_with_config(&config.player),
        grounded_state: GroundedState { is_grounded: false },
        ground_detection: GroundDetection {
            ray_distance: 0.6,
        },
        ground_ray: GroundRay::default(),
        dynamic_damping: DynamicDamping::default(),
        mesh: Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5)))),
        material: MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.9),
            metallic: 0.1,
            perceptual_roughness: 0.3,
            reflectance: 0.5,
            ..default()
        })),
        transform: Transform::from_xyz(
            initial_player_pos.x,
            initial_player_pos.y,
            initial_player_pos.z,
        ),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(0.25, 0.25, 0.25),
        external_force: ExternalForce::default(),
        velocity: Velocity::default(),
        restitution: Restitution::coefficient(0.0),
        friction: Friction::coefficient(0.3),
        damping: Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        locked_axes: LockedAxes::ROTATION_LOCKED,
        gravity_scale: GravityScale(1.0),
    });
}
