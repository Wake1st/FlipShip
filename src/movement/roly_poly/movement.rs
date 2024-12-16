use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::RolyPolyDirection;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::RolyPoly
}

fn store_input(
    controller: Res<Controller>,
    mut roly_poly_direction: Query<&mut RolyPolyDirection>,
) {
    if let Ok(mut direction) = roly_poly_direction.get_single_mut() {
        direction.value = Vec3::new(controller.right_stick.y, 0.0, -controller.right_stick.x);
    }
}

fn process_movement(
    roly_poly_direction: Query<&RolyPolyDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
) {
    if let Ok(direction) = roly_poly_direction.get_single() {
        if let Ok(mut ship_transform) = ship.get_single_mut() {
            //	tumble based on the ever changing size of the ship
        }
    }
}
