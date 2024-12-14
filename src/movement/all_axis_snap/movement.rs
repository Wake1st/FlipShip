use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{AllAxisDirection, ALL_AXIS_SNAP_TRANSLATION_SPEED, QUARTER_TURN_ROTATION};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::AllAxisSnap
}

fn store_input(controller: Res<Controller>, mut all_direction: Query<&mut AllAxisDirection>) {
    if let Ok(mut direction) = all_direction.get_single_mut() {
        //  translation
        direction.frontback = match controller.left_stick.y {
            -1.0..0.0 => -1.0,
            0.0 => 0.0,
            0.0..1.0 => 1.0,
            _ => 0.0,
        };
        direction.sideways = match controller.left_stick.x {
            -1.0..0.0 => -1.0,
            0.0 => 0.0,
            0.0..1.0 => 1.0,
            _ => 0.0,
        };
        direction.updown = match (controller.left_button, controller.bottom_button) {
            (true, true) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
            (false, false) => 0.0,
        };

        //  rotation
        direction.pitch = match controller.right_stick.y {
            -1.0..0.0 => 1.0,
            0.0 => 0.0,
            0.0..1.0 => -1.0,
            _ => 0.0,
        };
        direction.roll = match controller.right_stick.x {
            -1.0..0.0 => -1.0,
            0.0 => 0.0,
            0.0..1.0 => 1.0,
            _ => 0.0,
        };
        direction.yaw = match (controller.left_bumper, controller.right_bumper) {
            (true, true) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
            (false, false) => 0.0,
        };
    }
}

fn process_movement(
    all_direction: Query<&AllAxisDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) {
    if let Ok(direction) = all_direction.get_single() {
        if let Ok(mut transform) = ship.get_single_mut() {
            //	store time change
            let delta_time = time.delta_secs();

            //	translation
            transform.translation +=
                Vec3::new(direction.sideways, direction.updown, -direction.frontback)
                    * ALL_AXIS_SNAP_TRANSLATION_SPEED
                    * delta_time;

            //	rotation - HOW THE FUCK DO CURVES WORK?
            EasingCurve::new(
                transform.rotation,
                transform.rotation.rotate_towards(
                    Quat::from_rotation_x(direction.pitch * QUARTER_TURN_ROTATION),
                    direction.pitch * QUARTER_TURN_ROTATION,
                ),
                EaseFunction::BackOut,
            );
            EasingCurve::new(
                transform.rotation,
                transform.rotation.rotate_towards(
                    Quat::from_rotation_y(direction.yaw * QUARTER_TURN_ROTATION),
                    direction.yaw * QUARTER_TURN_ROTATION,
                ),
                EaseFunction::BackOut,
            );
            EasingCurve::new(
                transform.rotation,
                transform.rotation.rotate_towards(
                    Quat::from_rotation_z(direction.roll * QUARTER_TURN_ROTATION),
                    direction.roll * QUARTER_TURN_ROTATION,
                ),
                EaseFunction::BackOut,
            );
            transform.rotate_local_x(direction.pitch * QUARTER_TURN_ROTATION);
            transform.rotate_local_y(direction.yaw * QUARTER_TURN_ROTATION);
            transform.rotate_local_z(-direction.roll * QUARTER_TURN_ROTATION);
        }
    }
}
