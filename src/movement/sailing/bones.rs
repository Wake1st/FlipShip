use std::f32::consts::PI;

use bevy::prelude::*;

pub const SPAN_RATE: f32 = 0.6;

pub const SAIL_TURNING_RATE: f32 = PI;
pub const MIN_SAIL_ANGLE: f32 = -PI / 2.0;
pub const MAX_SAIL_ANGLE: f32 = PI / 2.0;

pub const RUDDER_TURNING_RATE: f32 = PI / 2.0;
pub const MIN_RUDDER_ANGLE: f32 = -PI / 3.0;
pub const MAX_RUDDER_ANGLE: f32 = PI / 3.0;

#[derive(Component)]
pub struct SailDirection {
    pub span: f32,
    pub sail_angle: f32,
    pub rudder_angle: f32,
}

#[derive(Component)]
pub struct Wind {
    pub direction: Vec2,
    pub magnitude: f32,
}
