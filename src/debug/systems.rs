use super::components::*;
use crate::camera::CameraAngle;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn toggle_debug_mode(
    mut debug_state: ResMut<DebugState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_state.enabled = !debug_state.enabled;
        debug_state.show_physics_debug = debug_state.enabled;
        debug_render_context.enabled = debug_state.show_physics_debug;
    }
}

pub fn setup_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            DebugOverlay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Auto,
                height: Val::Auto,
                min_width: Val::Px(200.0),
                min_height: Val::Px(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                DebugText,
                Text::new("Debug Mode (F3 to toggle)"),
                TextFont {
                    font: asset_server.load("fonts/doto/Doto-VariableFont_ROND,wght.ttf"),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn toggle_debug_visibility(
    debug_state: Res<DebugState>,
    mut debug_overlay_query: Query<&mut Visibility, (With<DebugOverlay>, Without<DebugText>)>,
) {
    for mut visibility in debug_overlay_query.iter_mut() {
        *visibility = if debug_state.enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn update_debug_text(
    debug_state: Res<DebugState>,
    mut debug_text_query: Query<&mut Text, With<DebugText>>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&CameraAngle>,
    entity_query: Query<Entity>,
    rigidbody_query: Query<&Velocity, With<RigidBody>>,
    time: Res<Time>,
) {
    if !debug_state.enabled {
        return;
    }

    for mut text in debug_text_query.iter_mut() {
        let mut debug_info = String::new();

        debug_info.push_str(
            "        _  _   __  _  _  ____  __      ____   __   _  _  ____   __        \n",
        );
        debug_info.push_str(
            "       / )( \\ /  \\( \\/ )(  __)(  )    / ___) / _\\ ( \\/ )(  _ \\ /  \\       \n",
        );
        debug_info.push_str(
            "       \\ \\/ /(  O ))  (  ) _) / (_/\\  \\___ \\/    \\/ \\/ \\ ) __/(  O )      \n",
        );
        debug_info.push_str(
            "        \\__/  \\__/(_/\\_)(____)\\_____/ (____/\\_/\\_/\\_)(_/(__)   \\__/       \n",
        );
        debug_info.push('\n');
        debug_info.push_str(&format!("v{}\n", env!("CARGO_PKG_VERSION")));
        debug_info.push_str("Debug Screen (F3 to toggle)\n\n");

        debug_info.push_str(&format!("FPS: {:.1}\n", 1.0 / time.delta().as_secs_f32()));
        debug_info.push_str(&format!("Entities: {}\n", entity_query.iter().count()));
        debug_info.push_str(&format!(
            "RigidBodies: {}\n\n",
            rigidbody_query.iter().count()
        ));

        if let Ok(player_transform) = player_query.single() {
            let pos = player_transform.translation;
            debug_info.push_str(&format!(
                "XYZ: {:.3} / {:.3} / {:.3}\n",
                pos.x, pos.y, pos.z
            ));
            debug_info.push_str(&format!(
                "Block: {} {} {}\n\n",
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32
            ));

            if let Ok(velocity) = rigidbody_query.single() {
                let vel = velocity.linvel;
                debug_info.push_str(&format!(
                    "Velocity: {:+07.3} / {:+07.3} / {:+07.3}\n",
                    vel.x, vel.y, vel.z
                ));
                debug_info.push_str(&format!("Speed: {:.3} m/s\n\n", vel.length()));
            }
        } else {
            debug_info.push_str("Player: Not found\n\n");
        }

        if let Ok(camera_angle) = camera_query.single() {
            let forward = camera_angle.get_camera_forward_direction();
            let facing = if forward.z < -0.5 {
                "North (-Z)"
            } else if forward.z > 0.5 {
                "South (+Z)"
            } else if forward.x > 0.5 {
                "East (+X)"
            } else if forward.x < -0.5 {
                "West (-X)"
            } else {
                "Unknown"
            };
            debug_info.push_str(&format!("Facing: {facing}\n"));
        } else {
            debug_info.push_str("Facing: Unknown\n");
        }

        **text = debug_info;
    }
}
