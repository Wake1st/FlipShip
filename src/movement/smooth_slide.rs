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
    movement_state.0 == MovementType::SmoothSlide
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  translation
    let frontback = -controller.left_stick.y;
    let sideways = controller.left_stick.x;
    let updown = match (controller.left_button, controller.bottom_button) {
        (true, true) => 0.0,
        (true, false) => 1.0,
        (false, true) => -1.0,
        (false, false) => 0.0,
    };

    direction.0 = Direction {
        translation_value: Vec3::new(sideways, updown, frontback),
        translation_flow: DirectionFlow::Controled(SMOOTH_SLIDE_TRANSLATION_SPEED),
        ..default()
    }
}
