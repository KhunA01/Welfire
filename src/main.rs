use bevy::{prelude::*, window::WindowMode};
use well_fire::systems::SystemsPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fantasy RPG".to_owned(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SystemsPlugins)
        .run();
}
