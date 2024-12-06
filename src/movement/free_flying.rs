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
    movement_state.0 == MovementType::FreeFlying
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  translation
    let forward_constant = 1.0;

    //  rotation
    let pitch = controller.right_stick.y;
    let yaw: f32 = controller.right_stick.x;

    direction.0 = Direction {
        translation_value: Vec3::new(0.0, 0.0, forward_constant),
        translation_flow: DirectionFlow::Varried(40.0, 80.0),
        rotation_value: Vec3::new(pitch, yaw, 0.0),
        rotation_flow: DirectionFlow::Controled(5.0),
    }
}
