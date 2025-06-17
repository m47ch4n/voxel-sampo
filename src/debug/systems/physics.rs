use crate::physics::DynamicDamping;
use crate::player::{GroundedState, Player};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Type alias to reduce complexity warnings

pub struct PhysicsInfo {
    pub is_grounded: bool,
    pub grounded_state: Option<GroundedState>,
    pub damping_config: Option<DynamicDamping>,
    pub current_damping: Option<Damping>,
    pub gravity_scale: Option<f32>,
    pub friction: Option<f32>,
    pub restitution: Option<f32>,
    pub present: bool,
}

impl PhysicsInfo {
    pub fn from_queries(
        player_query: &Query<
            (
                &GroundedState,
                &DynamicDamping,
                &Damping,
                &GravityScale,
                &Friction,
                &Restitution,
            ),
            With<Player>,
        >,
    ) -> Self {
        if let Ok((
            grounded_state,
            dynamic_damping,
            damping,
            gravity_scale,
            friction,
            restitution,
        )) = player_query.single()
        {
            Self {
                is_grounded: grounded_state.is_grounded,
                grounded_state: Some(grounded_state.clone()),
                damping_config: Some(dynamic_damping.clone()),
                current_damping: Some(*damping),
                gravity_scale: Some(gravity_scale.0),
                friction: Some(friction.coefficient),
                restitution: Some(restitution.coefficient),
                present: true,
            }
        } else {
            Self {
                is_grounded: false,
                grounded_state: None,
                damping_config: None,
                current_damping: None,
                gravity_scale: None,
                friction: None,
                restitution: None,
                present: false,
            }
        }
    }

    pub fn format(&self) -> String {
        if !self.present {
            return "Physics: Not found".to_string();
        }

        let mut info = format!("Grounded: {}", if self.is_grounded { "Yes" } else { "No" });

        if let Some(grounded_state) = &self.grounded_state {
            info.push_str(&format!(
                "\nGround Ray:\n  Origin: {:?}\n  Direction: {:?}\n  Distance: {:.2}\n  Hit Point: {:?}",
                grounded_state.ray_origin, grounded_state.ray_direction, grounded_state.ray_distance, grounded_state.hit_point
            ));
        }

        if let Some(damping) = &self.current_damping {
            info.push_str(&format!(
                "\nDamping: Linear {:.1}, Angular {:.1}",
                damping.linear_damping, damping.angular_damping
            ));
        }

        if let Some(gravity) = self.gravity_scale {
            info.push_str(&format!("\nGravity Scale: {gravity:.1}"));
        }

        if let Some(friction) = self.friction {
            info.push_str(&format!("\nFriction: {friction:.2}"));
        }

        if let Some(restitution) = self.restitution {
            info.push_str(&format!("\nRestitution: {restitution:.2}"));
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
    ray_query: Query<&GroundedState, With<Player>>,
    debug_state: Res<super::super::components::DebugState>,
    mut gizmos: Gizmos,
) {
    if !debug_state.enabled {
        return;
    }

    for grounded_state in ray_query.iter() {
        let ray_end = if let Some(hit_point) = grounded_state.hit_point {
            hit_point
        } else {
            grounded_state.ray_origin + grounded_state.ray_direction * grounded_state.ray_distance
        };

        let color = if grounded_state.is_grounded {
            LinearRgba::GREEN
        } else {
            LinearRgba::RED
        };

        gizmos.line(grounded_state.ray_origin, ray_end, color);
    }
}
