use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementState(MovementType::AllAxisSnap));
    }
}

#[derive(Debug, PartialEq)]
pub enum MovementType {
    AllAxisSnap,
    SmoothSlide,
    ForwardFlying,
    FreeFlying,
    RolyPoly,
    WiggleWalk,
    Tank,
    Sailing,
    TunnelTwist,
}

#[derive(Resource, Debug)]
pub struct MovementState(pub MovementType);
