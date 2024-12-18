mod demo;
mod movement;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((demo::plugin, movement::plugin));
    }
}
