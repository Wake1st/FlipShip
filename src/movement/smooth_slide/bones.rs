use bevy::prelude::*;

pub const SMOOTH_SLIDE_TRANSLATION_SPEED: f32 = 60.0;

#[derive(Component)]
pub struct SlideDirection {
    pub value: Vec3,
}
