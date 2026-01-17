use bevy::prelude::*;
use crate::resources::player_res::{PlayerAnimations, PlayerModels};

pub fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let animations: PlayerAnimations = PlayerAnimations {
        idle: asset_server.load("animations/Sword And Shield Idle.glb#Animation0"),
        fw_walk: asset_server.load("animations/Sword And Shield Walk.glb#Animation0"),
        bw_walk: asset_server.load("animations/Sword And Shield Walk Back.glb#Animation0"),
        left_walk: asset_server.load("animations/Sword And Shield Strafe Right.glb#Animation0"),
        right_walk: asset_server.load("animations/Sword And Shield Strafe Left.glb#Animation0"),
        fw_run: asset_server.load("animations/Sword And Shield Run.glb#Animation0"),
        bw_run: asset_server.load("animations/Sword And Shield Run Back.glb#Animation0"),
        run_left: asset_server.load("animations/Sword And Shield Run Left.glb#Animation0"),
        run_right: asset_server.load("animations/Sword And Shield Run Right.glb#Animation0"),
        attack: asset_server.load("animations/Sword And Shield Attack.glb#Animation0"),
        slash: asset_server.load("animations/Sword And Shield Slash.glb#Animation0"),
        jump: asset_server.load("animations/Sword And Shield Jump.glb#Animation0"),
        kick: asset_server.load("animations/Sword And Shield Kick.glb#Animation0"),
        crouch: asset_server.load("animations/Sword And Shield Crouch.glb#Animation0")
    };

    let models: PlayerModels = PlayerModels {
        warrior: asset_server.load("models/Warrior.glb#Scene0"),
    };

    commands.insert_resource(animations);
    commands.insert_resource(models);
}
