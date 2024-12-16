use bevy::prelude::*;

pub const SLIDE_RATE: f32 = 30.0;
pub const FORWARD_RATE: f32 = 20.0;

const HALF_HEIGHT_BOUNDARY: f32 = 400.0;
const HALF_WIDTH_BOUNDARY: f32 = 600.0;
pub const MIN_BOUNDARY: Vec2 = Vec2::new(-HALF_WIDTH_BOUNDARY, -HALF_HEIGHT_BOUNDARY);
pub const MAX_BOUNDARY: Vec2 = Vec2::new(HALF_WIDTH_BOUNDARY, HALF_HEIGHT_BOUNDARY);

#[derive(Component)]
pub struct ForwardFlyingDirection {
    pub slide: Vec2,
}
