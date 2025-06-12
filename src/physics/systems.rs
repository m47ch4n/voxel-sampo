use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::{DynamicDamping, GroundDetection, GroundRay};
use crate::player::{GroundedState, Player};

pub fn ground_detection_system(
    mut query: Query<(Entity, &mut GroundedState, &Transform, &GroundDetection, &mut GroundRay), With<Player>>,
    rapier_context: ReadRapierContext,
) {
    if let Ok(context) = rapier_context.single() {
        for (entity, mut grounded_state, transform, ground_detection, mut ground_ray) in query.iter_mut() {
            let ray_pos = transform.translation;
            let ray_dir = Vec3::NEG_Y;
            let max_toi = ground_detection.ray_distance;
            let solid = true;
            let filter = QueryFilter::default().exclude_collider(entity);

            let hit_result = context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter);
            grounded_state.is_grounded = hit_result.is_some();

            ground_ray.origin = ray_pos;
            ground_ray.direction = ray_dir;
            ground_ray.distance = max_toi;
            ground_ray.hit_point = hit_result.map(|(_, toi)| ray_pos + ray_dir * toi);
        }
    }
}

pub fn dynamic_damping_system(
    mut query: Query<(&GroundedState, &mut Damping, &DynamicDamping), With<Player>>,
) {
    for (grounded_state, mut damping, dynamic_damping) in query.iter_mut() {
        if grounded_state.is_grounded {
            damping.linear_damping = dynamic_damping.ground_damping;
        } else {
            damping.linear_damping = dynamic_damping.air_damping;
        }
    }
}

pub fn player_rotation_lock_system(mut player_query: Query<&mut Transform, With<Player>>) {
    for mut transform in player_query.iter_mut() {
        transform.rotation = Quat::IDENTITY;
    }
}
