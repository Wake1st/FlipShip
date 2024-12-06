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
    movement_state.0 == MovementType::Tank
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  translation
    let forward = controller.left_stick.y + controller.right_stick.y;

    //  rotation
    let yaw = -controller.left_stick.y + controller.right_stick.y;

    direction.0 = Direction {
        translation_value: Vec3::new(0.0, 0.0, forward),
        translation_flow: DirectionFlow::Controled(forward),
        rotation_value: Vec3::new(0.0, yaw, 0.0),
        rotation_flow: DirectionFlow::Controled(yaw),
    }
}
