pub const TRACK_TO_TRACK_MOTION_ARM: f32 = 10.0;

#[derive(Component)]
pub struct TankBase {
    right_track: f32,
    left_track: f32,
}