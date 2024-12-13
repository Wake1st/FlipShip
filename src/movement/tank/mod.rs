mod movement;
mod bones;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        movement::plugin,
    );
}
