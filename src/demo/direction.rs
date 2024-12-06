use bevy::prelude::*;

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionState(Direction { ..default() }));
    }
}

#[derive(Debug)]
pub enum DirectionFlow {
    None,
    Constant(bool),
    Varried(f32, f32),
    Controled(f32),
    Incremental(f32),
}

#[derive(Debug)]
pub struct Direction {
    pub translation_value: Vec3,
    pub translation_flow: DirectionFlow,
    pub rotation_value: Vec3,
    pub rotation_flow: DirectionFlow,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            translation_value: Vec3::ZERO,
            translation_flow: DirectionFlow::None,
            rotation_value: Vec3::ZERO,
            rotation_flow: DirectionFlow::None,
        }
    }
}

#[derive(Resource, Debug)]
pub struct DirectionState(pub Direction);
