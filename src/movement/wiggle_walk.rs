use bevy::prelude::*;

use crate::demo::{
    constants::SMOOTH_SLIDE_TRANSLATION_SPEED,
    controls::Controller,
    direction::{Direction, DirectionFlow, DirectionState},
    movement::{MovementState, MovementType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_movement.run_if(is_matching_state));
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::ForwardFlying
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    todo!()
}
