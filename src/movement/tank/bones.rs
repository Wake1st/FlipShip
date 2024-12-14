use bevy::prelude::*;

pub const TRACK_TO_TRACK_MOTION_ARM: f32 = 10.0;

#[derive(Component)]
pub struct TankBase {
    pub right_track: f32,
    pub left_track: f32,
}
