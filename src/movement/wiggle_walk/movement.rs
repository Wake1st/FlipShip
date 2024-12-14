use bevy::prelude::*;

use crate::demo::{
    controls::Controller,
    movement::{MovementState, MovementType},
};

use super::bones::{
    FeetMovement, Foot, Leg, Torso, UnitFeet, FAR_FOOT_MOTION_ARM, FOOT_TORSO_LENGTH, LEG_LENGTH,
    NEAR_FOOT_MOTION_ARM, TORSO_MOTION_ARM,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (foot_input, animate_walking)
            .chain()
            .run_if(is_matching_state),
    );
}

fn is_matching_state(movement_state: Res<MovementState>) -> bool {
    movement_state.0 == MovementType::ForwardFlying
}

///    Stores usable movement value from user input
///    TODO: might feel "weird" for the trigger and stick to be on the same side;
///    might want to switch them - criss-cross.
///    Q: would it be more fun to allow "slipping"? I think so. (might not work so well with the controls)
fn foot_input(
    controller: Res<Controller>,
    mut unit_feet: ResMut<UnitFeet>,
    mut feet_movement: ResMut<FeetMovement>,
) {
    if controller.right_trigger {
        let difference = controller.right_stick - unit_feet.right;
        unit_feet.right += controller.right_stick;
        feet_movement.moving = difference;
        feet_movement.left = false;
    } else if controller.left_trigger {
        let difference = controller.left_stick - unit_feet.left;
        unit_feet.left += controller.left_stick;
        feet_movement.moving = difference;
        feet_movement.left = true;
    }
}

fn animate_walking(
    feet_movement: Res<FeetMovement>,
    mut torso_query: Query<&mut Transform, With<Torso>>,
    mut leg_query: Query<(&mut Transform, &Leg)>,
    mut foot_query: Query<(&mut Transform, &Foot)>,
) {
    //  store horizontal movement of foot
    let foot_movement_vec3 = Vec3::new(feet_movement.moving.x, 0.0, -feet_movement.moving.y);
    let horizontal_translation = feet_movement.moving * LEG_LENGTH;
    let om_normal = (feet_movement.moving - feet_movement.stable).normalize();

    //  store leg rotation values
    let mut torso_translation_delta: Vec3 = Vec3::ZERO;

    //  move feet
    for (mut foot_transform, foot) in foot_query.iter_mut() {
        //  calculate movement
        if (foot.left && feet_movement.left) || (!foot.left && !feet_movement.left) {
            foot_transform.translation += foot_movement_vec3;
        }
    }

    //  move torso
    if let Ok(mut torso_transform) = torso_query.get_single_mut() {
        //  translate
        torso_translation_delta = calculate_translation(
            om_normal,
            horizontal_translation,
            TORSO_MOTION_ARM,
            FOOT_TORSO_LENGTH,
        );
        torso_transform.translation += torso_translation_delta;

        //  rotate
        let y_pivot = calculate_y_rotation(torso_transform.forward(), om_normal);
        torso_transform.rotate_local_y(y_pivot);
    }

    //  move legs
    for (mut leg_transform, leg) in leg_query.iter_mut() {
        //  translation
        let left_motion_arm = if feet_movement.left {
            FAR_FOOT_MOTION_ARM
        } else {
            NEAR_FOOT_MOTION_ARM
        };
        let right_motion_arm = if feet_movement.left {
            NEAR_FOOT_MOTION_ARM
        } else {
            FAR_FOOT_MOTION_ARM
        };

        let torso_delta_normal = torso_translation_delta.normalize();
        let z_pivot = torso_delta_normal.angle_between(leg_transform.up().into());
        if leg.left {
            leg_transform.translation += calculate_translation(
                om_normal,
                horizontal_translation,
                left_motion_arm,
                LEG_LENGTH,
            );
            leg_transform.rotate_local_z(z_pivot);
        } else {
            leg_transform.translation += calculate_translation(
                om_normal,
                horizontal_translation,
                right_motion_arm,
                LEG_LENGTH,
            );
            leg_transform.rotate_local_z(-z_pivot);
        }

        //  rotation
        let y_pivot = calculate_y_rotation(leg_transform.forward(), om_normal);
        leg_transform.rotate_local_y(y_pivot);
    }
}

fn calculate_translation(
    om_normal: Vec2,
    horizontal_translation: Vec2,
    horizontal_motion_arm: f32,
    vertical_motion_arm: f32,
) -> Vec3 {
    let radial_projection = -om_normal.dot(horizontal_translation * horizontal_motion_arm);
    let direction = match radial_projection {
        -1.0..0.0 => -1.0,
        0.0 => 0.0,
        0.0..1.0 => 1.0,
        _ => 0.0,
    };
    let vertical_translation =
        direction * (vertical_motion_arm - radial_projection * radial_projection).sqrt();
    Vec3::new(
        horizontal_translation.x * 0.5,
        vertical_translation,
        -horizontal_translation.y * 0.5,
    )
}

fn calculate_y_rotation(forward: Dir3, normal: Vec2) -> f32 {
    Vec3::new(normal.x, 0.0, -normal.y).angle_between(forward.into())
}
