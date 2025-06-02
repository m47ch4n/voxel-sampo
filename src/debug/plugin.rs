use bevy::prelude::*;
use super::components::*;
use super::systems::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<DebugState>()
        .add_systems(Startup, setup_debug_ui)
        .add_systems(Update, (
            toggle_debug_mode,
            update_debug_info,
        ));
} 