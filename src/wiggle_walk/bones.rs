use bevy::prelude::*;

pub const FOOT_TORSO_LENGTH: f32 = 10.0;
pub const TORSO_HIP_LENGTH: f32 = 5.0;
pub const LEG_LENGTH: f32 = 5.0;
pub const NEAR_FOOT_MOTION_ARM: f32 = 0.25;
pub const FAR_FOOT_MOTION_ARM: f32 = 0.75;
pub const TORSO_MOTION_ARM: f32 = 0.5;

#[derive(Component)]
pub struct Hip {
    pub left: bool,
}

#[derive(Component)]
pub struct Foot {
    pub left: bool,
}

#[derive(Component)]
pub struct Leg {
    pub left: bool,
}

#[derive(Resource)]
pub struct UnitFeet {
    pub left: Vec2,
    pub right: Vec2,
}

#[derive(Resource)]
pub struct FeetMovement {
    pub moving: Vec2,
    pub stable: Vec2,
    pub left: bool,
}
