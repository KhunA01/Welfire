use bevy::prelude::*;

pub mod player_system;
pub mod camera_system;
pub mod world_system;
pub mod loading_system;
pub use loading_system::*;
pub use world_system::*;
pub use player_system::*;
pub use camera_system::*;

pub struct SystemsPlugins;

impl Plugin for SystemsPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_player_assets)
            .add_systems(Startup, (spawn_world, spawn_player))
            .add_systems(Update, (toggle_cursor, move_player, scene_animation, animate_player))
            .add_systems(PostUpdate, update_camera_position);
    }
}
