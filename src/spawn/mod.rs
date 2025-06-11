use bevy::prelude::*;

use crate::config::Config;
use crate::entities::{camera::spawn_camera, player::spawn_player};

pub fn spawn_entities(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<Config>,
) {
    spawn_player(&mut commands, &mut meshes, &mut materials, &config);
    spawn_camera(&mut commands, &config);
}
