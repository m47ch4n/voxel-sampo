use super::super::components::*;
use super::{
    fps::FpsInfo, performance::PerformanceInfo, physics::PhysicsInfo, player::PlayerInfo,
    world::WorldInfo,
};
use crate::camera::CameraAngle;
use crate::physics::{DynamicDamping, GroundDetection};
use crate::player::{GroundedState, Player};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update_debug_text(
    debug_state: Res<DebugState>,
    mut debug_text_query: Query<&mut Text, With<DebugText>>,
    player_query: Query<&Transform, With<Player>>,
    physics_query: Query<
        (
            &GroundedState,
            &GroundDetection,
            &DynamicDamping,
            &Damping,
            &GravityScale,
            &Friction,
            &Restitution,
        ),
        With<Player>,
    >,
    camera_query: Query<&CameraAngle>,
    entity_query: Query<Entity>,
    rigidbody_query: Query<&Velocity, With<RigidBody>>,
    time: Res<Time>,
) {
    if !debug_state.enabled {
        return;
    }

    for mut text in debug_text_query.iter_mut() {
        let debug_info = compose_debug_info(
            &time,
            &entity_query,
            &rigidbody_query,
            &player_query,
            &physics_query,
            &camera_query,
        );
        **text = debug_info;
    }
}

fn compose_debug_info(
    time: &Time,
    entity_query: &Query<Entity>,
    rigidbody_query: &Query<&Velocity, With<RigidBody>>,
    player_query: &Query<&Transform, With<Player>>,
    physics_query: &Query<
        (
            &GroundedState,
            &GroundDetection,
            &DynamicDamping,
            &Damping,
            &GravityScale,
            &Friction,
            &Restitution,
        ),
        With<Player>,
    >,
    camera_query: &Query<&CameraAngle>,
) -> String {
    let mut debug_info = String::new();

    // Header section
    debug_info.push_str(&get_header_section());
    debug_info.push('\n');

    // Performance section
    let fps_info = FpsInfo::from_time(time);
    let performance_info = PerformanceInfo::from_queries(entity_query, rigidbody_query);
    debug_info.push_str(&fps_info.format());
    debug_info.push('\n');
    debug_info.push_str(&performance_info.format());
    debug_info.push_str("\n\n");

    // Player section
    let player_info = PlayerInfo::from_queries(player_query, rigidbody_query);
    debug_info.push_str(&player_info.format());
    debug_info.push_str("\n\n");

    // Physics section
    let physics_info = PhysicsInfo::from_queries(physics_query);
    debug_info.push_str(&physics_info.format());
    debug_info.push_str("\n\n");

    // World section
    let world_info = WorldInfo::from_camera_query(camera_query);
    debug_info.push_str(&world_info.format());

    debug_info
}

fn get_header_section() -> String {
    let mut header = String::new();
    header.push_str(
        "        _  _   __  _  _  ____  __      ____   __   _  _  ____   __        \n",
    );
    header.push_str(
        "       / )( \\ /  \\( \\/ )(  __)(  )    / ___) / _\\ ( \\/ )(  _ \\ /  \\       \n",
    );
    header.push_str(
        "       \\ \\/ /(  O ))  (  ) _) / (_/\\  \\___ \\/    \\/ \\/ \\ ) __/(  O )      \n",
    );
    header.push_str(
        "        \\__/  \\__/(_/\\_)(____)\\_____/ (____/\\_/\\_/\\_)(_/(__)   \\__/       \n",
    );
    header.push('\n');
    header.push_str(&format!("v{}\n", env!("CARGO_PKG_VERSION")));
    header.push_str("Debug Screen (F3 to toggle)\n");
    header
}