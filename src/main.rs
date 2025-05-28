use bevy::{prelude::*, window::WindowResolution};
use bevy_vox_scene::VoxScenePlugin;

mod camera;
mod config;
mod player;
mod spawn;

use config::Config;
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
        VoxScenePlugin::default(),
        player::plugin,
        camera::plugin,
    ))
    .init_resource::<Config>()
    .add_systems(Startup, spawn_entities);

    app.run();
}
