use bevy::prelude::*;

mod all_axis_snap;
mod forward_flying;
mod free_flying;
mod roly_poly;
mod sailing;
mod smooth_slide;
mod tank;
mod tunnel_twist;
mod wiggle_walk;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        all_axis_snap::plugin,
        forward_flying::plugin,
        free_flying::plugin,
        roly_poly::plugin,
        sailing::plugin,
        smooth_slide::plugin,
        tank::plugin,
        tunnel_twist::plugin,
        wiggle_walk::plugin,
    ));
}
