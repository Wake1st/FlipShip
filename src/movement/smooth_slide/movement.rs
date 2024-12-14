use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{SlideDirection, SMOOTH_SLIDE_TRANSLATION_SPEED};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::SmoothSlide
}

fn store_input(controller: Res<Controller>, mut slide_direction: Query<&mut SlideDirection>) {
    if let Ok(mut direction) = slide_direction.get_single_mut() {
        direction.value = Vec3::new(
            controller.left_stick.x,
            match (controller.left_button, controller.bottom_button) {
                (true, true) => 0.0,
                (true, false) => 1.0,
                (false, true) => -1.0,
                (false, false) => 0.0,
            },
            -controller.left_stick.y,
        );
    }
}

fn process_movement(
    slide_direction: Query<&SlideDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) {
    //  translation
    if let Ok(direction) = slide_direction.get_single() {
        if let Ok(mut transform) = ship.get_single_mut() {
            transform.translation +=
                direction.value * SMOOTH_SLIDE_TRANSLATION_SPEED * time.delta_secs();
        }
    }
}
