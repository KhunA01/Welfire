use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub target_distance: f32,
    pub pivot_y: f32,
    pub offset_x: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            current_distance: 4.0,
            target_distance: 4.0,
            pivot_y: 1.5,
            offset_x: 0.5,
        }
    }
}