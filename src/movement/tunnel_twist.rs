use bevy::prelude::*;

use crate::demo::{
    constants::QUARTER_TURN_ROTATION,
    controls::Controller,
    direction::{Direction, DirectionFlow, DirectionState},
    movement::{MovementState, MovementType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_movement.run_if(is_matching_state));
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::TunnelTwist
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  translation
    let forward_constant = 1.0;

    //  rotation
    let pivot = controller.right_stick;

    direction.0 = Direction {
        translation_value: Vec3::new(0.0, 0.0, forward_constant),
        translation_flow: DirectionFlow::Constant(true),
        rotation_value: Vec3::new(pivot.x, pivot.y, 0.0),
        rotation_flow: DirectionFlow::Incremental(QUARTER_TURN_ROTATION),
    }
}
