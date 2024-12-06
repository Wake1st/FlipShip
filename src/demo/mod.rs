use bevy::prelude::*;
use controls::ControlsPlugin;
use movement::MovementPlugin;
use ship::ShipPlugin;

mod controls;
mod movement;
mod ship;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ShipPlugin, MovementPlugin, ControlsPlugin));
}
