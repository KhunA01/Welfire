use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerAnimations {
    pub idle: Handle<AnimationClip>,
    pub fw_walk: Handle<AnimationClip>,
    pub bw_walk: Handle<AnimationClip>,
    pub left_walk: Handle<AnimationClip>,
    pub right_walk: Handle<AnimationClip>,
    pub fw_run: Handle<AnimationClip>,
    pub bw_run: Handle<AnimationClip>,
    pub run_left: Handle<AnimationClip>,
    pub run_right: Handle<AnimationClip>,
    pub jump: Handle<AnimationClip>,
    pub attack: Handle<AnimationClip>,
    pub slash: Handle<AnimationClip>,
    pub kick: Handle<AnimationClip>,
    pub crouch: Handle<AnimationClip>
}

#[derive(Resource, Default)]
pub struct PlayerModels {
    pub warrior: Handle<Scene>,
}