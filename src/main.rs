use bevy::prelude::*;
use lib::AppPlugin;

mod lib;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppPlugin)
        .run();
}
