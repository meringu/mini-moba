use bevy::{prelude::*, window::PrimaryWindow};

use crate::{click::ClickEvent, player::PlayerCommand};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    camera_query: Query<(&Camera, &mut GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut ev_click: EventWriter<ClickEvent>,
    mut ev_player: EventWriter<PlayerCommand>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    if buttons.just_pressed(MouseButton::Right) {
        if let Some(pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_global_transform, pos) {
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) {
                    let point = ray.get_point(distance);
                    ev_click.send(ClickEvent(point));
                    ev_player.send(PlayerCommand::Move(point).into());
                }
            }
        }
    }
}
