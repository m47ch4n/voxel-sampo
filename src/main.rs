use bevy::{
    prelude::*,
    window::WindowResolution,
};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_vox_scene::VoxScenePlugin;

mod config;
mod player;
mod camera;
mod spawn;

use config::GameSettings;
use spawn::spawn_entities;

fn main() {
    let mut app = App::new();

    let window_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Voxel Sampo".to_string(),
            resolution: WindowResolution::new(960., 720.).with_scale_factor_override(1.),
            resizable: false,
            ..default()
        }),
        ..default()
    });

    app.add_plugins((
            window_plugins,
            PanOrbitCameraPlugin,
            VoxScenePlugin,
            player::plugin,
            camera::plugin,
        ))
        .init_resource::<GameSettings>()
        .add_systems(Startup, spawn_entities);

    app.run();
}
