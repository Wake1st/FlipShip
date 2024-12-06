use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins();
    }
}

#[derive(Component)]
pub struct Ship {}
