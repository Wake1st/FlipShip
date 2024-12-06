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
    movement_state.0 == MovementType::RolyPoly
}

fn process_movement(controller: Res<Controller>, mut direction: ResMut<DirectionState>) {
    //  rotation
    let pitch = controller.right_stick.y;
    let roll: f32 = controller.right_stick.x;

    direction.0 = Direction {
        rotation_value: Vec3::new(pitch, 0.0, roll),
        rotation_flow: DirectionFlow::Controled(60.0),
        ..default()
    }
}
