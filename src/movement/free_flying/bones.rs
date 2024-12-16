use bevy::prelude::*;

pub const ACCELERATION: f32 = 12.0;
pub const MAX_SPEED: f32 = 80.0;
pub const MIN_SPEED: f32 = 50.0;
pub const YAW_TURN_RATE: f32 = 5.0;
pub const PITCH_TURN_RATE: f32 = 3.0;

#[derive(Component)]
pub struct FreeFlyingDirection {
    pub speed: f32,
    pub turn_rate: Vec2,
}
