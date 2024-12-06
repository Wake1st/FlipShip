use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins();
    }
}

pub enum MovementType {
    AllAxisSnap,
    SideSlide,
    ForwardFlying,
    FreeFlying,
    RolyPoly,
    WiggleWalk,
    Tank,
    Sailing,
    TunnelTwist,
}
