use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::DynamicDamping;
use crate::player::{GroundedState, Player, RayInfo};

const GROUND_RAY_OFFSET_EPS: f32 = 2e-2;
const GROUND_RAY_DISTANCE: f32 = 1e-1; // ε = 10cm

pub fn ground_detection_system(
    mut query: Query<(Entity, &mut GroundedState, &Transform, &Collider), With<Player>>,
    rapier_context: ReadRapierContext,
) {
    if let Ok(context) = rapier_context.single() {
        for (entity, mut grounded_state, transform, collider) in query.iter_mut() {
            let bottom_y = if let Some(cuboid) = collider.as_cuboid() {
                transform.translation.y - cuboid.half_extents().y
            } else {
                println!(
                    "Warning: Non-cuboid collider detected, using center position as fallback"
                );
                transform.translation.y
            };

            // Create 9-point grid for ground detection
            let half_extent = if let Some(cuboid) = collider.as_cuboid() {
                // Use a fraction of the collider size for the grid spacing
                Vec2::new(cuboid.half_extents().x * 0.8, cuboid.half_extents().z * 0.8)
            } else {
                Vec2::new(0.4, 0.4) // fallback size
            };

            // Generate 9 ray positions in a 3x3 grid
            let ray_positions = [
                // Center
                Vec3::new(
                    transform.translation.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z,
                ),
                // 8 surrounding points
                Vec3::new(
                    transform.translation.x - half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z - half_extent.y,
                ), // SW
                Vec3::new(
                    transform.translation.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z - half_extent.y,
                ), // S
                Vec3::new(
                    transform.translation.x + half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z - half_extent.y,
                ), // SE
                Vec3::new(
                    transform.translation.x - half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z,
                ), // W
                Vec3::new(
                    transform.translation.x + half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z,
                ), // E
                Vec3::new(
                    transform.translation.x - half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z + half_extent.y,
                ), // NW
                Vec3::new(
                    transform.translation.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z + half_extent.y,
                ), // N
                Vec3::new(
                    transform.translation.x + half_extent.x,
                    bottom_y + GROUND_RAY_OFFSET_EPS,
                    transform.translation.z + half_extent.y,
                ), // NE
            ];

            let ray_dir = Vec3::NEG_Y;
            let max_toi = GROUND_RAY_DISTANCE;
            let solid = true;
            let filter = QueryFilter::default().exclude_collider(entity);

            // Cast rays and collect results
            let mut rays = Vec::with_capacity(9);
            let mut hit_count = 0;

            for &ray_pos in &ray_positions {
                let hit_result = context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter);
                let hit_point = hit_result.map(|(_, toi)| ray_pos + ray_dir * toi);

                if hit_result.is_some() {
                    hit_count += 1;
                }

                rays.push(RayInfo {
                    origin: ray_pos,
                    direction: ray_dir,
                    distance: max_toi,
                    hit: hit_point,
                });
            }

            // Update grounded state
            grounded_state.is_grounded = hit_count > 0;
            grounded_state.rays = rays;
            grounded_state.hit_count = hit_count;
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
