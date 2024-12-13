use bevy::prelude::*;

use crate::demo::{
    constants::QUARTER_TURN_ROTATION,
    controls::Controller,
    movement::{MovementState, MovementType},
};

use super::bones::{TankBase, TRACK_TO_TRACK_MOTION_ARM};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, move_tank.run_if(is_matching_state && track_input));
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::TunnelTwist
}

fn track_input(controller: Res<Controller>, mut tank_base: Query<&mut TankBase>) -> bool {
    if let Ok(base) = tank_base.get_single_mut() {
        //  store input
        base.right_track = controller.right_stick.y;
        base.left_track = controller.left_stick.y;

        //  return true for any movement
        base.left_track != 0.0 || base.right_track != 0.0
    } else {
        false
    }
}

///    TODO: should add a Player component to specify what we are controlling
fn move_tank(tank_base: Query<&TankBase>, mut player: Query<&mut Transform>, time: Res<Time>) {
    if let Ok(base) = tank_base.get_single() {
        if let Ok(transform) = player.get_single_mut() {
            //  store time
            let delta_time = time.delta_seconds();

            //  translation
            let forward_movement = 0.5 * base.left_track + 0.5 * base.right_track;
            transform.translation += Vec3::new(0.0, 0.0, forward_movement) * delta_time;

            //  rotation
            let angle = (base.right_track/TRACK_TO_TRACK_MOTION_ARM).atan() - (base.left_track/TRACK_TO_TRACK_MOTION_ARM).atan();
            transform.rotate_local_y(angle * delta_time);
        }
    }
}