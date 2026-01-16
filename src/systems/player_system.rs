use std::{time::Duration};

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{
    components::{
        camera::CameraController,
        player::{PlayerAnimationIndices, PlayerComponent},
    },
    resources::{PlayerAnimations, PlayerModels},
};

pub fn spawn_player(
    mut commands: Commands,
    mut cursor_system: Query<&mut CursorOptions>,
    models: Res<PlayerModels>,
    animations: Res<PlayerAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if let Ok(mut cursor) = cursor_system.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    let mut graph: AnimationGraph = AnimationGraph::new();
    let idle_idx = graph.add_clip(animations.idle.clone(), 1.0, graph.root);
    let fw_walk_idx = graph.add_clip(animations.fw_walk.clone(), 1.0, graph.root);
    let bw_walk_idx = graph.add_clip(animations.bw_walk.clone(), 1.0, graph.root);
    let left_walk_idx = graph.add_clip(animations.left_walk.clone(), 1.0, graph.root);
    let right_walk_idx = graph.add_clip(animations.right_walk.clone(), 1.0, graph.root);
    let graph_handle: Handle<AnimationGraph> = graphs.add(graph);

    commands.spawn((
        PlayerComponent,
        SceneRoot(models.warrior.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AnimationGraphHandle(graph_handle),
        PlayerAnimationIndices {
            idle: idle_idx,
            fw_walk: fw_walk_idx,
            bw_walk: bw_walk_idx,
            left_walk: left_walk_idx,
            right_walk: right_walk_idx,
        },
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    camera_query: Query<&CameraController>,
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

    if input.pressed(KeyCode::KeyW) {
        direction -= forward_vec;
    }
    if input.pressed(KeyCode::KeyS) {
        direction += forward_vec;
    }
    if input.pressed(KeyCode::KeyD) {
        direction -= right_vec;
    }
    if input.pressed(KeyCode::KeyA) {
        direction += right_vec;
    }

    let speed: f32 = 5.0;
    if direction.length() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * speed * time.delta_secs();
    }
    player_transform.rotation = Quat::from_rotation_y(cam_controller.yaw + std::f32::consts::PI);
}

pub fn scene_animation(
    mut commands: Commands,
    mut players: Query<Entity, Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    animation_config: Query<(&PlayerAnimationIndices, &AnimationGraphHandle)>,
) {
    for entity in &mut players {
        for ancestor in parents.iter_ancestors(entity) {
            if let Ok((indices, graph_handle)) = animation_config.get(ancestor) {
                commands.entity(entity).insert(graph_handle.clone());
                commands.entity(entity).insert(indices.clone());
                commands
                    .entity(entity)
                    .insert(AnimationTransitions::default());
            }
        }
    }
}

pub fn animate_player(
    mut players: Query<(
        &mut AnimationPlayer,
        &mut AnimationTransitions,
        &PlayerAnimationIndices,
    )>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut player, mut transitions, indices) in &mut players {
        let mut target_animation = indices.idle;

        if input.pressed(KeyCode::KeyW) {
            target_animation = indices.fw_walk;
        } else if input.pressed(KeyCode::KeyS) {
            target_animation = indices.bw_walk;
        } else if input.pressed(KeyCode::KeyA) {
            target_animation = indices.left_walk;
        } else if input.pressed(KeyCode::KeyD) {
            target_animation = indices.right_walk;
        }

        if !player.is_playing_animation(target_animation) {
            transitions
                .play(&mut player, target_animation, Duration::from_millis(250))
                .repeat();
        }
    }
}
