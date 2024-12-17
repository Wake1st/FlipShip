use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
};

use super::bones::{TunnelCenter, TwistDirection, FORWARD_RATE, HORIZONTAL_RATE};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::TunnelTwist
}

fn store_input(
    controller: Res<Controller>,
    mut twist_direction: Query<&mut TwistDirection>,
    time: Res<Time>,
) {
    if let Ok(mut direction) = twist_direction.get_single_mut() {
        direction.horizontal += controller.right_stick.x * HORIZONTAL_RATE * time.delta_secs();
    }
}

fn process_movement(
    twist_direction: Query<&mut TwistDirection>,
    mut parent: Query<&mut Transform, With<TunnelCenter>>,
    time: Res<Time>,
) {
    if let Ok(direction) = twist_direction.get_single() {
        if let Ok(mut transform) = parent.get_single_mut() {
            //	rotation
            transform.rotate_local_z(direction.horizontal);

            //  translation
            let rotation = transform.rotation;
            transform.translation +=
                rotation * Vec3::new(0.0, 0.0, FORWARD_RATE * time.delta_secs());
        }
    }
}
