use bevy::prelude::*;

use super::systems::setup_world;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_world);
}
