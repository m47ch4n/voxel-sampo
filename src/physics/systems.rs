use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::DynamicDamping;
use crate::player::{GroundedState, Player};

const GROUND_RAY_OFFSET_EPS: f32 = 2e-2;
const GROUND_RAY_DISTANCE: f32 = 1e-1; // ε = 10cm

pub fn ground_detection_system(
    mut query: Query<(Entity, &mut GroundedState, &Transform, &Collider), With<Player>>,
    rapier_context: ReadRapierContext,
) {
    if let Ok(context) = rapier_context.single() {
        for (entity, mut grounded_state, transform, collider) in query.iter_mut() {
            let bottom_y = if let Some(cuboid) = collider.as_cuboid() {
                // bottom = center_y - half_extent_y
                transform.translation.y - cuboid.half_extents().y
            } else {
                println!(
                    "Warning: Non-cuboid collider detected, using center position as fallback"
                );
                transform.translation.y
            };

            // ray_start = bottom + ε to account for physics engine penetration
            let ray_pos = Vec3::new(
                transform.translation.x,
                bottom_y + GROUND_RAY_OFFSET_EPS,
                transform.translation.z,
            );
            let ray_dir = Vec3::NEG_Y;
            let max_toi = GROUND_RAY_DISTANCE;
            let solid = true;
            let filter = QueryFilter::default().exclude_collider(entity);

            let hit_result = context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter);

            grounded_state.is_grounded = hit_result.is_some();
            grounded_state.ray_origin = ray_pos;
            grounded_state.ray_direction = ray_dir;
            grounded_state.ray_distance = max_toi;
            grounded_state.hit_point = hit_result.map(|(_, toi)| ray_pos + ray_dir * toi);
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
