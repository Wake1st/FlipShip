use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Controller { ..default() });
    }
}

#[derive(Resource, Default, Debug)]
pub struct Controller {
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub left_bumper: bool,
    pub right_bumper: bool,
    pub left_trigger: bool,
    pub right_trigger: bool,
    pub right_button: bool,
    pub bottom_button: bool,
    pub left_button: bool,
    pub top_button: bool,
    pub menu_button: bool,
}
