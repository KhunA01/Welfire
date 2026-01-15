use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct PlayerAnimationIndices {
    pub idle: AnimationNodeIndex,
    pub fw_walk: AnimationNodeIndex,
    // pub bw_walk: AnimationNodeIndex,
    // pub fw_run: AnimationNodeIndex,
    // pub bw_run: AnimationNodeIndex,
    // pub run_left: AnimationNodeIndex,
    // pub run_right: AnimationNodeIndex,
    // pub jump: AnimationNodeIndex,
    // pub attack: AnimationNodeIndex,
    // pub slash: AnimationNodeIndex,
    // pub kick: AnimationNodeIndex,
}