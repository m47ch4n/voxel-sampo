use super::components::*;
use super::systems::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<DebugState>()
        .add_systems(Startup, setup_debug_ui)
        .add_systems(
            Update,
            (
                toggle_debug_mode,
                toggle_debug_visibility,
                update_debug_text,
            ),
        );
}
