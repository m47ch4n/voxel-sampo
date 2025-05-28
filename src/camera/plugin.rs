use super::systems::{
    camera_follow_system, camera_input_system, camera_rotation_system, camera_zoom_system,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            camera_input_system,
            camera_follow_system.after(camera_input_system),
            camera_zoom_system.after(camera_follow_system),
            camera_rotation_system.after(camera_follow_system),
        ),
    );
}
