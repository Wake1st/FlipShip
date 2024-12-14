use std::f32::consts::PI;

use bevy::prelude::*;

pub const ALL_AXIS_SNAP_TRANSLATION_SPEED: f32 = 1.0;
pub const QUARTER_TURN_ROTATION: f32 = PI / 2.0;

///    Component assumes a positive forward, positive right, and positive up axis
#[derive(Component)]
pub struct AllAxisDirection {
    pub frontback: f32,
    pub sideways: f32,
    pub updown: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
