use bevy::prelude::*;

use crate::entities::world::{spawn_lighting, spawn_room};

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_room(&mut commands, &asset_server);
    spawn_lighting(&mut commands, &asset_server);
}
