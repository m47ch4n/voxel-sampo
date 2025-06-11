use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::physics::{DynamicDamping, GroundDetection};
use crate::player::{GroundedState, Player};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub grounded_state: GroundedState,
    pub ground_detection: GroundDetection,
    pub dynamic_damping: DynamicDamping,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub friction: Friction,
    pub damping: Damping,
    pub locked_axes: LockedAxes,
    pub gravity_scale: GravityScale,
}
