use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{ForwardFlyingDirection, FORWARD_RATE, MAX_BOUNDARY, MIN_BOUNDARY, SLIDE_RATE};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::ForwardFlying
}

fn store_input(
    controller: Res<Controller>,
    mut flying_direction: Query<&mut ForwardFlyingDirection>,
) {
    if let Ok(mut direction) = flying_direction.get_single_mut() {
        direction.slide = controller.right_stick;
    }
}

fn process_movement(
    flying_direction: Query<&ForwardFlyingDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) {
    if let Ok(direction) = flying_direction.get_single() {
        if let Ok(mut transform) = ship.get_single_mut() {
            //	translation
            transform.translation += (direction.slide * SLIDE_RATE)
                .clamp(MIN_BOUNDARY, MAX_BOUNDARY)
                .extend(FORWARD_RATE)
                * time.delta_secs();
        }
    }
}
