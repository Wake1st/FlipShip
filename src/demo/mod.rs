use bevy::prelude::*;
use controls::ControlsPlugin;
use direction::DirectionPlugin;
use movement::MovementPlugin;
use ship::ShipPlugin;

pub mod constants;
pub mod controls;
pub mod direction;
pub mod movement;
mod ship;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ShipPlugin, MovementPlugin, ControlsPlugin, DirectionPlugin));
}
