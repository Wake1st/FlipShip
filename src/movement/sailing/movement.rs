use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{
    SailDirection, Wind, MAX_RUDDER_ANGLE, MAX_SAIL_ANGLE, MIN_RUDDER_ANGLE, MIN_SAIL_ANGLE,
    RUDDER_TURNING_RATE, SAIL_TURNING_RATE, SPAN_RATE,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (store_input, process_movement)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::Sailing
}

fn store_input(
    controller: Res<Controller>,
    mut sail_direction: Query<&mut SailDirection>,
    time: Res<Time>,
) {
    if let Ok(mut direction) = sail_direction.get_single_mut() {
        let delta_time = time.delta_secs();

        //	span
        let span_direction = match (controller.right_trigger, controller.right_bumper) {
            (true, true) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
            (false, false) => 0.0,
        };
        let current_span = direction.span;
        direction.span = (current_span + span_direction * SPAN_RATE * delta_time).clamp(0.0, 1.0);

        //	sail angle - 0 is "at rest", where the sail is parallel to the length of the ship
        let current_angle = direction.sail_angle;
        direction.sail_angle = (current_angle
            + controller.right_stick.x * SAIL_TURNING_RATE * delta_time)
            .clamp(MIN_SAIL_ANGLE, MAX_SAIL_ANGLE);

        //	rudder angle
        let current_angle = direction.rudder_angle;
        direction.rudder_angle = (current_angle
            + controller.left_stick.x * RUDDER_TURNING_RATE * delta_time)
            .clamp(MIN_RUDDER_ANGLE, MAX_RUDDER_ANGLE);
    }
}

fn process_movement(
    sail_direction: Query<&SailDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
    wind_query: Query<&Wind>,
) {
    if let Ok(direction) = sail_direction.get_single() {
        if let Ok(wind) = wind_query.get_single() {
            if let Ok(mut ship_transform) = ship.get_single_mut() {
                //	obtain the thrust from the sails
                let ship_wise_wind = wind
                    .direction
                    .extend(0.0)
                    .dot_into_vec(ship_transform.forward().into());

                let angle = direction.sail_angle;
                let sail_forward = if angle < 0.0 {
                    -direction.sail_angle
                } else {
                    direction.sail_angle
                };
                let ship_wise_sail = Vec3::new(sail_forward.cos(), sail_forward.sin(), 0.0)
                    .dot_into_vec(ship_transform.forward().into());
                let thrust = ship_wise_wind.dot(ship_wise_sail) * wind.magnitude;

                //	rotate ship
                ship_transform.rotate_local_y(thrust * -direction.rudder_angle);
                let ship_rotation = ship_transform.rotation;

                //	translate
                let ship_forward: Vec3 = ship_transform.forward().into();
                ship_transform.translation +=
                    ship_rotation * (ship_forward * direction.span * thrust.max(0.0));
            }
        }
    }
}
