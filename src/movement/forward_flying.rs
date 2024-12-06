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
    movement_state.0 == MovementType::ForwardFlying
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  translation
    let forward_constant = 1.0;
    let updown = controller.left_stick.y;
    let sideways = controller.left_stick.x;

    //  rotation
    let pitch = controller.right_stick.y;
    let roll = controller.right_stick.x;
    let yaw = match (controller.left_bumper, controller.right_bumper) {
        (true, true) => 0.0,
        (true, false) => 1.0,
        (false, true) => -1.0,
        (false, false) => 0.0,
    };

    direction.0 = Direction {
        translation_value: Vec3::new(sideways, updown, forward_constant),
        translation_flow: DirectionFlow::Controled(15.0),
        rotation_value: Vec3::new(pitch, yaw, roll),
        rotation_flow: DirectionFlow::Incremental(QUARTER_TURN_ROTATION),
    };
}
