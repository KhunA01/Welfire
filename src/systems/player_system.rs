use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{components::{camera::CameraController, player::{PlayerAnimationIndices, PlayerComponent}}, resources::{PlayerAnimations, PlayerModels}};

pub fn spawn_player(
    mut commands: Commands,
    mut cursor_system: Query<&mut CursorOptions>,
    models: Res<PlayerModels>,
    animations: Res<PlayerAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>
) {
    if let Ok(mut cursor) = cursor_system.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    let mut graph: AnimationGraph = AnimationGraph::new();
    let idle_idx = graph.add_clip(animations.idle.clone(), 1.0, graph.root);
    let fw_walk_idx = graph.add_clip(animations.fw_walk.clone(), 1.0, graph.root);
    let graph_handle: Handle<AnimationGraph> = graphs.add(graph);

    commands.spawn((
        PlayerComponent,
        SceneRoot(models.warrior.clone()),
        AnimationGraphHandle(graph_handle),
        AnimationPlayer::default(),
        PlayerAnimationIndices {
            idle: idle_idx,
            fw_walk: fw_walk_idx,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    camera_query: Query<&CameraController>,
    mut animation_query: Query<(&PlayerAnimationIndices, &mut AnimationPlayer), With<PlayerComponent>>,
) {
    let cam_controller: &CameraController = match camera_query.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut player_transform: Mut<'_, Transform> = match player_query.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut direction: Vec3 = Vec3::ZERO;
    let forward: f32 = cam_controller.yaw;
    let right: f32 = cam_controller.yaw - std::f32::consts::FRAC_PI_2;
    let forward_vec: Vec3 = Vec3::new(forward.sin(), 0.0, forward.cos());
    let right_vec: Vec3 = Vec3::new(right.sin(), 0.0, right.cos());

    if input.pressed(KeyCode::KeyW) { direction -= forward_vec; }
    if input.pressed(KeyCode::KeyS) { direction += forward_vec; }
    if input.pressed(KeyCode::KeyD) { direction -= right_vec; }
    if input.pressed(KeyCode::KeyA) { direction += right_vec; }

    let speed: f32 = 5.0;
    if direction.length() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * speed * time.delta_secs();
    }
    player_transform.rotation = Quat::from_rotation_y(cam_controller.yaw);

    if let Ok((indices, mut player)) = animation_query.single_mut() {
        
        if direction.length() > 0.0 {
            // ถ้ามีการเดิน -> สั่งเล่นท่าวิ่ง (run_idx)
            // .play() จะเล่นท่าใหม่ทันที
            // .repeat() สั่งให้เล่นวนลูป
            if !player.is_playing_animation(indices.fw_walk) {
                player.play(indices.fw_walk).repeat();
            }
        } else {
            // ถ้าหยุดเดิน -> สั่งเล่นท่ายืน (idle_idx)
            if !player.is_playing_animation(indices.idle) {
                player.play(indices.idle).repeat();
            }
        }
    }
}
