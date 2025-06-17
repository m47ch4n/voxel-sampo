use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PerformanceInfo {
    pub entity_count: usize,
    pub rigidbody_count: usize,
}

impl PerformanceInfo {
    pub fn from_queries(
        entity_query: &Query<Entity>,
        rigidbody_query: &Query<&Velocity, With<RigidBody>>,
    ) -> Self {
        Self {
            entity_count: entity_query.iter().count(),
            rigidbody_count: rigidbody_query.iter().count(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "Entities: {}\nRigidBodies: {}",
            self.entity_count, self.rigidbody_count
        )
    }
}
