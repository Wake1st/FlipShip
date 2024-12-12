use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    direction::{Direction, DirectionFlow, DirectionState},
    movement::{MovementState, MovementType},
};

use super::bones::{FeetMovement, UnitFeet, FAR_FOOT_MOTION_ARM, FOOT_TORSO_LENGTH, LEG_LENGTH, NEAR_FOOT_MOTION_ARM, TORSO_MOTION_ARM};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_walking.run_if(is_matching_state && foot_input));
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::ForwardFlying
}

///    Stores usable movement value from user input
///    TODO: might feel "weird" for the trigger and stick to be on the same side;
///    might want to switch them - criss-cross.
///    Q: would it be more fun to allow "slipping"? I think so. (might not work so well with the controls)
fn foot_input(controller: Res<Controller>, mut unit_feet: ResMut<UnitFeet>, mut feet_movement: ResMut<FeetMovement>) -> bool {
    if controller.right_trigger {
        let difference = controller.right_stick - unit_feet.right;
        unit_feet.right += controller.right_stick;
        feet_movement.moving = difference;
        feet_movement.left = false;
        true
    } else if controller.left_trigger {
        let difference = controller.left_stick - unit_feet.left;
        unit_feet.left += controller.left_stick;
        feet_movement.moving = difference;
        feet_movement.left = true;
        true
    } else {
        false
    }
}

fn animate_walking(
    feet_movement: Res<FeetMovement>, 
    mut torso_query: Query<&mut Transform, With<Torso>>,
    mut hip_query: Query<(&mut Transform, &Hip)>,
    mut leg_query: Query<(&mut Transform, &Leg)>, 
    mut foot_query: Query<(&mut Transform, &Foot)>,
) {
    //  store horizontal movement of foot
    let horizontal_translation = feet_movement.moving * LEG_LENGTH;
    let om_normal = feet_movement.om_normal();

    //  store leg rotation values
    let mut torso_translation_delta: Vec3;

    //  move feet
    for (mut foot_transform, foot) in foot_query.iter_mut() {
        //  calculate movement
        if (foot.left && feet_movement.left) || (!foot.left && !feet_movement.left) {
            foot_transform.translation += feet_movement.moving;
        }
    }

    //  move torso
    if let Ok(mut torso_transform) = torso_query.get_single_mut() {
        //  translate
        torso_translation_delta = calculate_translation(TORSO_MOTION_ARM, FOOT_TORSO_LENGTH);
        torso_transform.translation += torso_translation_delta;

        //  rotate
        let y_pivot = calculate_y_rotation(torso_transform.FORWARD, om_normal);
        torso_transform.rotate_around(y_pivot, Vec3::UP);
    }

    //  move legs
    for (mut leg_transform, leg) in leg_query.iter_mut() {
        //  pre-calculate FOWARD rotation values
        let old_foot_hip = old_hip_translation - old_foot_translation;

        //  translation
        let left_motion_arm = if feet_movement.left { FAR_FOOT_MOTION_ARM } else { NEAR_FOOT_MOTION_ARM };
        let right_motion_arm = if feet_movement.left { NEAR_FOOT_MOTION_ARM } else { FAR_FOOT_MOTION_ARM };

        if leg.left {
            leg_transform.translation += calculate_translation(left_motion_arm, LEG_LENGTH);
            leg_transform.rotate_around(torso_translation_delta, Vec3::FORWARD);
        } else {
            leg_transform.translation += calculate_translation(right_motion_arm, LEG_LENGTH);
            leg_transform.rotate_around(torso_translation_delta, Vec3::FORWARD);
        }

        //  rotation
        let y_pivot = calculate_y_rotation(leg_transform.FORWARD, om_normal);
        leg_transform.rotate_around(y_pivot, Vec3::UP);
    }
}

fn calculate_translation(horizontal_motion_arm: f32, vertical_motion_arm: f32) -> Vec3 {
    let radial_projection = -om_normal.dot(horizontal_translation * horizontal_motion_arm);
    let direction = match radial_projection {
        -1.0..0.0 => -1.0,
        0.0 => 0.0,
        0.0..1.0 => 1.0,
    };
    let vertical_translation = direction * (vertical_motion_arm - radial_projection * radial_projection).sqrt();
    Vec3::new(
        horizontal_translation.x * 0.5,
        vertical_translation,
        -horizontal_translation.z * 0.5,
    );
}

fn calculate_y_rotation(forward: Vec3, normal: Vec2) -> f32 {
    Vec2::new(-om_normal.y, om_normal.x, 0.0) - forward
}
