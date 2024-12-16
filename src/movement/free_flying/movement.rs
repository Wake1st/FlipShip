use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{
    FreeFlyingDirection, ACCELERATION, MAX_SPEED, MIN_SPEED, PITCH_TURN_RATE, YAW_TURN_RATE,
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
    movement_state.0 == MovementType::FreeFlying
}

fn store_input(
    controller: Res<Controller>,
    mut flying_direction: Query<&mut FreeFlyingDirection>,
    time: Res<Time>,
) {
    if let Ok(mut direction) = flying_direction.get_single_mut() {
        let delta_time = time.delta_secs();

        //	set speed
        let acceleration_direction = match (controller.right_trigger, controller.right_bumper) {
            (true, true) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
            (false, false) => 0.0,
        };
        let current_speed = direction.speed;
        direction.speed = (current_speed + acceleration_direction * ACCELERATION * delta_time)
            .clamp(MIN_SPEED, MAX_SPEED);

        //	set direction
        direction.turn_rate = Vec2::new(
            controller.right_stick.x * YAW_TURN_RATE * delta_time,
            -controller.right_stick.y * PITCH_TURN_RATE * delta_time,
        );
    }
}

fn process_movement(
    flying_direction: Query<&FreeFlyingDirection>,
    mut ship: Query<&mut Transform, With<Ship>>,
) {
    if let Ok(direction) = flying_direction.get_single() {
        if let Ok(mut transform) = ship.get_single_mut() {
            //	rotation
            transform.rotate_local_y(direction.turn_rate.x);
            transform.rotate_local_x(direction.turn_rate.y);

            //	translation
            let rotation = transform.rotation;
            transform.translation += rotation * Vec3::new(0.0, 0.0, -direction.speed);
        }
    }
}
