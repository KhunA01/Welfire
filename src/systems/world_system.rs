use std::f32::consts::PI;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::components::camera::CameraController;

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_system: Query<&mut CursorOptions>,
) {
    if let Ok(mut cursor) = cursor_system.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::default(),
    ));

    commands.spawn((
        DirectionalLight { illuminance: 5000.0, shadows_enabled: true, ..default()},
        Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::from(Srgba::rgb(81.0 / 255.0, 81.0 / 255.0, 81.0 / 255.0 )))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
