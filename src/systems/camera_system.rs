use crate::components::{camera::CameraController, player::PlayerComponent};
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

pub fn update_camera_position(
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
    player_query: Query<&Transform, (With<PlayerComponent>, Without<CameraController>)>,
    window_query: Query<&CursorOptions>,
) {
    let is_cursor_locked: bool = if let Ok(window) = window_query.single() {
        window.grab_mode == CursorGrabMode::Locked
    } else {
        false
    };

    let player_transform: &Transform = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let (mut cam_transform, mut controller) = match camera_query.single_mut() {
        Ok(vals) => vals,
        Err(_) => return,
    };

    if mouse_button.pressed(MouseButton::Right) {
        controller.target_distance = 1.0;
        // controller.offset_x = 0.6;
    } else {
        controller.target_distance = 4.0;
        // controller.offset_x = 0.8;
    }

    let zoom_speed: f32 = 10.0;
    controller.current_distance = controller
        .current_distance
        .lerp(controller.target_distance, time.delta_secs() * zoom_speed);

    if is_cursor_locked {
        let mut delta: Vec2 = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        if delta != Vec2::ZERO {
            let sensitivity: f32 = 0.002;
            controller.yaw -= delta.x * sensitivity;
            controller.pitch -= delta.y * sensitivity;
            controller.pitch = controller.pitch.clamp(-1.2, 1.2);
        }
    }

    let rotation: Quat = Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
    cam_transform.rotation = rotation;

    let pivot_point: Vec3 = player_transform.translation + Vec3::new(0.0, controller.pivot_y, 0.0);
    let local_offset: Vec3 = Vec3::new(controller.offset_x, 0.0, controller.current_distance);
    let final_position: Vec3 = pivot_point + (rotation * local_offset);
    // let follow_speed: f32 = 50.0;
    // let t: f32 = 1.0 - (-follow_speed * time.delta_secs()).exp();
    // cam_transform.translation = cam_transform
    //     .translation
    //     .lerp(final_position,  follow_speed * time.delta_secs());
    cam_transform.translation = final_position;
}

pub fn toggle_cursor(input: Res<ButtonInput<KeyCode>>, mut window_query: Query<&mut CursorOptions>) {
    if input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = window_query.single_mut() {
            if window.visible {
                window.grab_mode = CursorGrabMode::Locked;
                window.visible = false;
            } else {
                window.grab_mode = CursorGrabMode::None;
                window.visible = true;
            }
        }
    }
}

