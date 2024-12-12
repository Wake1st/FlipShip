mod demo;
mod movement;
mod wiggle_walk;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((demo::plugin, movement::plugin, wiggle_walk::plugin));
    }
}
