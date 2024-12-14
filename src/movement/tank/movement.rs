use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
    ship::Ship,
};

use super::bones::{TankBase, TRACK_TO_TRACK_MOTION_ARM};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (track_input, move_tank).chain().run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::TunnelTwist
}

fn track_input(controller: Res<Controller>, mut tank_base: Query<&mut TankBase>) {
    if let Ok(mut base) = tank_base.get_single_mut() {
        //  store input
        base.right_track = controller.right_stick.y;
        base.left_track = controller.left_stick.y;
    }
}

///    TODO: should add a Player component to specify what we are controlling
fn move_tank(
    tank_base: Query<&TankBase>,
    mut player: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) {
    if let Ok(base) = tank_base.get_single() {
        if let Ok(mut transform) = player.get_single_mut() {
            //  store time
            let delta_time = time.delta_secs();

            //  translation
            let forward_movement = 0.5 * base.left_track + 0.5 * base.right_track;
            transform.translation += Vec3::new(0.0, 0.0, forward_movement) * delta_time;

            //  rotation
            let angle = (base.right_track / TRACK_TO_TRACK_MOTION_ARM).atan()
                - (base.left_track / TRACK_TO_TRACK_MOTION_ARM).atan();
            transform.rotate_local_y(angle * delta_time);
        }
    }
}
