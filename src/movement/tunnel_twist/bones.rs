use bevy::prelude::*;

pub const FORWARD_RATE: f32 = 60.0;
pub const HORIZONTAL_RATE: f32 = 20.0;

#[derive(Component)]
pub struct TwistDirection {
    pub horizontal: f32,
}

#[derive(Component)]
pub struct TunnelCenter {}
