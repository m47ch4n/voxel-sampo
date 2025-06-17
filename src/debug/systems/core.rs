use super::super::components::*;
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
