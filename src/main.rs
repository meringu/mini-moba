mod camera;
mod click;
mod input;
mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(click::ClickPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(input::InputPlugin)
        .run();
}
