use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    direction::{Direction, DirectionFlow, DirectionState},
    movement::{MovementState, MovementType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_movement.run_if(is_matching_state));
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::Sailing
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  transition
    let sail = controller.right_stick;

    //  rotation
    let speed = match (controller.left_button, controller.bottom_button) {
        (true, true) => 0.0,
        (true, false) => 1.0,
        (false, true) => -1.0,
        (false, false) => 0.0,
    };

    direction.0 = Direction {
        translation_value: Vec3::new(0.0, 0.0, speed),
        translation_flow: DirectionFlow::Varried(0.0, 90.0),
        rotation_value: Vec3::new(sail.x, 0.0, sail.y),
        rotation_flow: DirectionFlow::Controled(10.0),
    }
}
