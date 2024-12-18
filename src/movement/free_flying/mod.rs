mod bones;
mod movement;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);
}
