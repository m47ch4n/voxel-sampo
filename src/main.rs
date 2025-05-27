use bevy::{
    core_pipeline::{
        bloom::BloomSettings, core_3d::ScreenSpaceTransmissionQuality, tonemapping::Tonemapping,
    },
    prelude::*,
    render::camera::ScalingMode,
    window::WindowResolution,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle};

// キャラクターコンポーネント
#[derive(Component)]
struct Player {
    move_timer: Timer,
    target_position: Vec3,
    is_moving: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            move_timer: Timer::from_seconds(0.3, TimerMode::Once), // カクカク動きのための移動時間
            target_position: Vec3::ZERO,
            is_moving: false,
        }
    }
}

// カメラコンポーネント
#[derive(Component)]
struct CameraController {
    current_position: usize,
    rotation_timer: Timer,
    is_rotating: bool,
    start_transform: Transform,
    target_transform: Transform,
    continuous_rotation: bool,
    rotation_direction: bool,
    current_angle: f32,
    rotation_speed: f32,
    snap_start_angle: f32, // スナップ開始時の角度
    continuous_rotation_elapsed: f32, // 連続回転開始からの経過時間（秒）
    rotation_acceleration_time: f32, // 0.2秒で最大速度に到達
    // 慣性制御
    is_decelerating: bool, // 減速中かどうか
    deceleration_elapsed: f32, // 減速開始からの経過時間
    deceleration_time: f32, // 減速にかかる時間
    current_velocity: f32, // 現在の回転速度（度/秒）
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            current_position: 0,
            rotation_timer: Timer::from_seconds(0.5, TimerMode::Once), // カメラ回転時間
            is_rotating: false,
            start_transform: Transform::IDENTITY,
            target_transform: Transform::IDENTITY,
            continuous_rotation: false,
            rotation_direction: false,
            current_angle: 60.0, // 初期角度
            rotation_speed: 360.0, // 360度/秒でより高速な回転
            snap_start_angle: 0.0, // スナップ開始時の角度
            continuous_rotation_elapsed: 0.0, // 経過時間を0で初期化
            rotation_acceleration_time: 0.2, // 0.2秒で最大速度に到達
            // 慣性制御の初期化
            is_decelerating: false,
            deceleration_elapsed: 0.0,
            deceleration_time: 0.4, // 0.4秒かけて減速
            current_velocity: 0.0,
        }
    }
}

impl CameraController {
    // 4つのカメラ位置を定義（FFTスタイル）
    fn get_camera_positions(player_pos: Vec3) -> [Transform; 4] {
        let distance = 12.0;
        let height = 8.0;
        let angles = [60.0f32, 150.0f32, 240.0f32, 330.0f32]; // 度数で定義

        angles.map(|angle| {
            let rad = angle.to_radians();
            Transform::from_xyz(
                player_pos.x + distance * rad.cos(), 
                player_pos.y + height, 
                player_pos.z + distance * rad.sin()
            ).looking_at(player_pos, Vec3::Y)
        })
    }

    fn get_current_transform(&self, player_pos: Vec3) -> Transform {
        Self::get_camera_positions(player_pos)[self.current_position]
    }
    
    fn get_camera_forward_direction(&self) -> Vec3 {
        let rad = self.current_angle.to_radians();
        
        // カメラが向いている方向（プレイヤーから見て奥方向）
        Vec3::new(-rad.cos(), 0.0, -rad.sin()).normalize()
    }
    
    fn get_camera_right_direction(&self) -> Vec3 {
        let forward = self.get_camera_forward_direction();
        forward.cross(Vec3::Y).normalize()
    }

    fn get_transform_from_angle(&self, player_pos: Vec3, angle: f32) -> Transform {
        let rad = angle.to_radians();
        Transform::from_xyz(
            player_pos.x + 12.0 * rad.cos(),
            player_pos.y + 8.0,
            player_pos.z + 12.0 * rad.sin()
        ).looking_at(player_pos, Vec3::Y)
    }
}

fn main() {
    let mut app = App::new();

    let custom_default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Voxel Sampo".to_string(),
            resolution: WindowResolution::new(960., 720.).with_scale_factor_override(1.),
            resizable: false,
            ..default()
        }),
        ..default()
    });

    app.add_plugins((custom_default_plugins, PanOrbitCameraPlugin, VoxScenePlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_input, player_movement, camera_input, camera_rotation),
        );

    app.run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_controller = CameraController::default();
    let initial_transform = camera_controller.get_current_transform(Vec3::ZERO);

    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(10.0),
                ..default()
            }
            .into(),
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::High,
                screen_space_specular_transmission_steps: 1,
                ..default()
            },
            transform: initial_transform,
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
            ..Default::default()
        },
        PanOrbitCamera::default(),
        camera_controller,
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: assets.load("papermill_diffuse.ktx2"),
            specular_map: assets.load("papermill_specular.ktx2"),
            intensity: 300.0,
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(1.0, -2.5, 0.85), Vec3::Y),
        ..default()
    });

    // ボクセルシーン
    commands.spawn(VoxelSceneBundle {
        scene: assets.load("room.vox"),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..default()
    });

    // プレイヤーキャラクター（直方体）
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.8, 1.6, 0.8)), // 人型っぽい直方体
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2), // 赤っぽい色
                perceptual_roughness: 0.8,
                metallic: 0.1,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.8, 0.0), // 地面から少し浮かせる
            ..default()
        },
        Player::default(),
    ));
}

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut player_query: Query<&mut Player>,
    camera_query: Query<&CameraController>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        // 移動中は新しい入力を受け付けない
        if player.is_moving {
            return;
        }

        let mut direction = Vec3::ZERO;
        
        // カメラの向きを取得
        if let Ok(camera_controller) = camera_query.get_single() {
            let forward = camera_controller.get_camera_forward_direction();
            let right = camera_controller.get_camera_right_direction();

            // WASDキーでカメラの向きに合わせた4方向移動
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += forward; // カメラから見て奥方向
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= forward; // カメラから見て手前方向
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction -= right; // カメラから見て左方向
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction += right; // カメラから見て右方向
            }
        }

        // 斜め移動を防ぐ（4方向のみ）
        if direction.length() > 0.0 {
            // 最も強い方向のみを採用
            if direction.x.abs() > direction.z.abs() {
                direction = Vec3::new(direction.x.signum(), 0.0, 0.0);
            } else {
                direction = Vec3::new(0.0, 0.0, direction.z.signum());
            }

            player.target_position += direction * 2.0; // 2ユニット移動
            player.is_moving = true;
            player.move_timer.reset();
        }
    }
}

fn player_movement(time: Res<Time>, mut player_query: Query<(&mut Player, &mut Transform)>) {
    if let Ok((mut player, mut transform)) = player_query.get_single_mut() {
        if player.is_moving {
            player.move_timer.tick(time.delta());

            let start_pos = transform.translation;
            let progress =
                player.move_timer.elapsed_secs() / player.move_timer.duration().as_secs_f32();

            if progress >= 1.0 {
                // 移動完了
                transform.translation = player.target_position;
                player.is_moving = false;
            } else {
                // 線形補間で移動（カクカク感を出すため）
                let eased_progress = if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                };
                transform.translation = start_pos.lerp(player.target_position, eased_progress);
            }
        }
    }
}

fn camera_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut CameraController>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    if let Ok(mut camera_controller) = camera_query.get_single_mut() {
        // E = 時計回り（長押し対応）
        if keyboard_input.pressed(KeyCode::KeyE) {
            if !camera_controller.continuous_rotation {
                camera_controller.continuous_rotation = true;
                camera_controller.rotation_direction = true;
                // 回転中の場合は現在の補間された角度を使用
                camera_controller.is_rotating = false; // スナップ回転を停止
                camera_controller.continuous_rotation_elapsed = 0.0; // 経過時間を0で初期化
                camera_controller.is_decelerating = false; // 減速を停止
            }
        }
        // Q = 反時計回り（長押し対応）
        else if keyboard_input.pressed(KeyCode::KeyQ) {
            if !camera_controller.continuous_rotation {
                camera_controller.continuous_rotation = true;
                camera_controller.rotation_direction = false;
                // 回転中の場合は現在の補間された角度を使用
                camera_controller.is_rotating = false; // スナップ回転を停止
                camera_controller.continuous_rotation_elapsed = 0.0; // 経過時間を0で初期化
                camera_controller.is_decelerating = false; // 減速を停止
            }
        }
        // QまたはEキーが離されたら慣性減速を開始
        else if keyboard_input.just_released(KeyCode::KeyQ) || keyboard_input.just_released(KeyCode::KeyE) {
            if camera_controller.continuous_rotation {
                camera_controller.continuous_rotation = false;
                camera_controller.is_decelerating = true;
                camera_controller.deceleration_elapsed = 0.0;
                
                // 現在の回転速度を記録（慣性の初期速度として使用）
                let acceleration_progress = (camera_controller.continuous_rotation_elapsed / camera_controller.rotation_acceleration_time).min(1.0);
                let speed_multiplier = acceleration_progress * acceleration_progress;
                let min_speed_ratio = 0.1;
                let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
                camera_controller.current_velocity = camera_controller.rotation_speed * final_speed_multiplier;
            }
        }
    }
}

fn camera_rotation(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraController, &mut Transform), With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    if let Ok((mut camera_controller, mut transform)) = camera_query.get_single_mut() {
        let player_pos = if let Ok(player_transform) = player_query.get_single() {
            player_transform.translation
        } else {
            Vec3::ZERO
        };

        // 連続回転の処理
        if camera_controller.continuous_rotation {
            // 経過時間を更新
            camera_controller.continuous_rotation_elapsed += time.delta_seconds();
            
            // 加速度を計算（ease-in カーブで滑らかに加速）
            let acceleration_progress = (camera_controller.continuous_rotation_elapsed / camera_controller.rotation_acceleration_time).min(1.0);
            let speed_multiplier = acceleration_progress * acceleration_progress; // シンプルなease-in
            
            // 最小速度を設定して完全に停止しないようにする
            let min_speed_ratio = 0.1; // 最低10%の速度は保持
            let final_speed_multiplier = min_speed_ratio + (1.0 - min_speed_ratio) * speed_multiplier;
            
            // 現在の回転速度を計算（滑らかに加速）
            let current_rotation_speed = camera_controller.rotation_speed * final_speed_multiplier;
            let rotation_delta = current_rotation_speed * time.delta_seconds();
            
            if camera_controller.rotation_direction {
                // 時計回り
                camera_controller.current_angle += rotation_delta;
            } else {
                // 反時計回り
                camera_controller.current_angle -= rotation_delta;
            }
            
            // 角度を0-360度の範囲に正規化
            camera_controller.current_angle = camera_controller.current_angle % 360.0;
            if camera_controller.current_angle < 0.0 {
                camera_controller.current_angle += 360.0;
            }
            
            // 現在の角度に基づいてカメラ位置を更新
            *transform = camera_controller.get_transform_from_angle(player_pos, camera_controller.current_angle);
        }
        // 慣性減速の処理
        else if camera_controller.is_decelerating {
            camera_controller.deceleration_elapsed += time.delta_seconds();
            
            // 減速カーブを計算（ease-out）
            let deceleration_progress = (camera_controller.deceleration_elapsed / camera_controller.deceleration_time).min(1.0);
            let remaining_speed_ratio = 1.0 - (deceleration_progress * deceleration_progress); // ease-out
            
            // 現在の速度を計算
            let current_speed = camera_controller.current_velocity * remaining_speed_ratio;
            let rotation_delta = current_speed * time.delta_seconds();
            
            if camera_controller.rotation_direction {
                // 時計回り
                camera_controller.current_angle += rotation_delta;
            } else {
                // 反時計回り
                camera_controller.current_angle -= rotation_delta;
            }
            
            // 角度を0-360度の範囲に正規化
            camera_controller.current_angle = camera_controller.current_angle % 360.0;
            if camera_controller.current_angle < 0.0 {
                camera_controller.current_angle += 360.0;
            }
            
            // 減速完了時の処理
            if deceleration_progress >= 1.0 {
                camera_controller.is_decelerating = false;
                
                // 最寄りの固定位置を見つけてスナップ
                let angles = [60.0f32, 150.0f32, 240.0f32, 330.0f32];
                let current_angle = camera_controller.current_angle;
                let mut closest_index = 0;
                let mut min_distance = f32::MAX;
                
                for (i, &angle) in angles.iter().enumerate() {
                    let distance = (angle - current_angle).abs();
                    let wrapped_distance = (360.0 - distance).min(distance);
                    if wrapped_distance < min_distance {
                        min_distance = wrapped_distance;
                        closest_index = i;
                    }
                }
                
                // 距離が十分小さい場合（5度以下）は直接スナップ、そうでなければアニメーション
                if min_distance <= 5.0 {
                    // 直接固定位置に設定
                    camera_controller.current_position = closest_index;
                    camera_controller.current_angle = angles[closest_index];
                    *transform = camera_controller.get_current_transform(player_pos);
                } else {
                    // 最寄りの固定位置にスナップアニメーションを開始
                    camera_controller.current_position = closest_index;
                    camera_controller.snap_start_angle = current_angle;
                    camera_controller.start_transform = camera_controller.get_transform_from_angle(player_pos, current_angle);
                    camera_controller.target_transform = camera_controller.get_current_transform(player_pos);
                    camera_controller.is_rotating = true;
                    camera_controller.rotation_timer.reset();
                }
            } else {
                // 現在の角度に基づいてカメラ位置を更新
                *transform = camera_controller.get_transform_from_angle(player_pos, camera_controller.current_angle);
            }
        }
        // 離散的な回転アニメーション（単発の回転用）
        else if camera_controller.is_rotating {
            camera_controller.rotation_timer.tick(time.delta());

            let progress = camera_controller.rotation_timer.elapsed_secs()
                / camera_controller.rotation_timer.duration().as_secs_f32();

            if progress >= 1.0 {
                // 回転完了
                *transform = camera_controller.target_transform;
                camera_controller.is_rotating = false;
                
                // 現在の角度を更新
                let angles = [60.0f32, 150.0f32, 240.0f32, 330.0f32];
                camera_controller.current_angle = angles[camera_controller.current_position];
            } else {
                // 角度ベースの補間で円周に沿った回転
                let angles = [60.0f32, 150.0f32, 240.0f32, 330.0f32];
                let target_angle = angles[camera_controller.current_position];
                
                // 記録された開始角度を使用
                let start_angle = camera_controller.snap_start_angle;
                
                // 角度差を計算（最短経路を使用）
                let mut angle_diff = target_angle - start_angle;
                
                // 最短経路になるように角度差を調整
                if angle_diff > 180.0 {
                    angle_diff -= 360.0;
                } else if angle_diff < -180.0 {
                    angle_diff += 360.0;
                }
                
                // イージング関数（ease-in-out）
                let eased_progress = if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                };
                
                // イージングされた進行度で現在の角度を計算
                let current_interpolated_angle = start_angle + angle_diff * eased_progress;
                
                // 角度を正規化
                let normalized_angle = if current_interpolated_angle < 0.0 {
                    current_interpolated_angle + 360.0
                } else if current_interpolated_angle >= 360.0 {
                    current_interpolated_angle - 360.0
                } else {
                    current_interpolated_angle
                };
                
                // 現在の角度を常に更新（連続回転への移行時のちらつき防止）
                camera_controller.current_angle = normalized_angle;
                
                // 補間された角度でカメラ位置を更新
                *transform = camera_controller.get_transform_from_angle(player_pos, normalized_angle);
            }
        }
        // カメラがプレイヤーを追従（回転中でない場合）
        else {
            *transform = camera_controller.get_transform_from_angle(player_pos, camera_controller.current_angle);
        }
    }
}
