use crate::player::{GroundedState, Player};
use crate::physics::{DynamicDamping, GroundDetection, GroundRay};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsInfo {
    pub is_grounded: bool,
    pub ground_detection_config: Option<GroundDetection>,
    pub damping_config: Option<DynamicDamping>,
    pub current_damping: Option<Damping>,
    pub gravity_scale: Option<f32>,
    pub friction: Option<f32>,
    pub restitution: Option<f32>,
    pub ground_ray: Option<GroundRay>,
    pub present: bool,
}

impl PhysicsInfo {
    pub fn from_queries(
        player_query: &Query<
            (
                &GroundedState,
                &GroundDetection,
                &DynamicDamping,
                &Damping,
                &GravityScale,
                &Friction,
                &Restitution,
                &GroundRay,
            ),
            With<Player>,
        >,
    ) -> Self {
        if let Ok((
            grounded_state,
            ground_detection,
            dynamic_damping,
            damping,
            gravity_scale,
            friction,
            restitution,
            ground_ray,
        )) = player_query.single()
        {
            Self {
                is_grounded: grounded_state.is_grounded,
                ground_detection_config: Some(ground_detection.clone()),
                damping_config: Some(dynamic_damping.clone()),
                current_damping: Some(damping.clone()),
                gravity_scale: Some(gravity_scale.0),
                friction: Some(friction.coefficient),
                restitution: Some(restitution.coefficient),
                present: true,
                ground_ray: Some(ground_ray.clone()),
            }
        } else {
            Self {
                is_grounded: false,
                ground_detection_config: None,
                damping_config: None,
                current_damping: None,
                gravity_scale: None,
                friction: None,
                restitution: None,
                present: false,
                ground_ray: None,
            }
        }
    }

    pub fn format(&self) -> String {
        if !self.present {
            return "Physics: Not found".to_string();
        }

        let mut info = format!("Grounded: {}", if self.is_grounded { "Yes" } else { "No" });

        if let Some(ground_detection) = &self.ground_detection_config {
            info.push_str(&format!(
                "\nGround Detection:\n  Ray Distance: {:.2}",
                ground_detection.ray_distance
            ));
        }

        if let Some(ground_ray) = &self.ground_ray {
            info.push_str(&format!(
                "\nGround Ray:\n  Origin: {:?}\n  Direction: {:?}\n  Distance: {:.2}\n  Hit Point: {:?}",
                ground_ray.origin, ground_ray.direction, ground_ray.distance, ground_ray.hit_point
            ));
        }

        if let Some(damping) = &self.current_damping {
            info.push_str(&format!(
                "\nDamping: Linear {:.1}, Angular {:.1}",
                damping.linear_damping, damping.angular_damping
            ));
        }

        if let Some(gravity) = self.gravity_scale {
            info.push_str(&format!("\nGravity Scale: {:.1}", gravity));
        }

        if let Some(friction) = self.friction {
            info.push_str(&format!("\nFriction: {:.2}", friction));
        }

        if let Some(restitution) = self.restitution {
            info.push_str(&format!("\nRestitution: {:.2}", restitution));
        }

        if let Some(damping_config) = &self.damping_config {
            info.push_str(&format!(
                "\nDamping Config:\n  Ground: {:.1}, Air: {:.1}",
                damping_config.ground_damping, damping_config.air_damping
            ));
        }

        info
    }
}

pub fn visualize_ground_rays_system(
    ray_query: Query<(&GroundRay, &GroundedState), With<Player>>,
    debug_state: Res<super::super::components::DebugState>,
    mut gizmos: Gizmos,
) {
    if !debug_state.enabled {
        return;
    }

    for (ground_ray, grounded_state) in ray_query.iter() {
        let ray_end = if let Some(hit_point) = ground_ray.hit_point {
            hit_point
        } else {
            ground_ray.origin + ground_ray.direction * ground_ray.distance
        };

        let color = if grounded_state.is_grounded {
            LinearRgba::GREEN
        } else {
            LinearRgba::RED
        };

        gizmos.line(ground_ray.origin, ray_end, color);
    }
}