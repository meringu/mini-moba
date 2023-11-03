use bevy::{prelude::*, window::PrimaryWindow};

use crate::click::ClickEvent;

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 10.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component, Default)]
struct Player {
    state: PlayerState,
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    Moving(Vec3),
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });

    commands.spawn(PlayerBundle {
        player: Player::default(),
        mesh: meshes.add(shape::UVSphere::default().into()),
        material: debug_material,
        transform: Transform::from_xyz(0.0, 1.0, 0.0),

        ..Default::default()
    });
}

fn update(
    mut query: Query<(&mut Transform, &mut Player)>,
    camera_query: Query<(&Camera, &mut GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut ev_click: EventWriter<crate::click::ClickEvent>,
) {
    let (mut player_transform, mut player) = query.single_mut();
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    // handle right click
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_global_transform, pos) {
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) {
                    let point = ray.get_point(distance);
                    ev_click.send(ClickEvent(point));
                    player.state = PlayerState::Moving(point);
                }
            }
        }
    }

    match player.state {
        PlayerState::Idle => {}
        PlayerState::Moving(location) => {
            // Only move player by x and z
            let mut target = location;
            target.y = player_transform.translation.y;

            let delta = target - player_transform.translation;

            if delta.length() == 0.0 {
                player.state = PlayerState::Idle;
            } else {
                player_transform.translation +=
                    delta.clamp_length(0.0, time.delta_seconds() * PLAYER_SPEED);
            }
        }
    }
}
