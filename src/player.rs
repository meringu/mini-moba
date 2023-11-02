use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 10.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default)]
struct PlayerTarget(Option<Vec3>);

#[derive(Bundle, Default)]
struct PlayerBundle {
    marker: Player,
    target: PlayerTarget,
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
        marker: Player,
        mesh: meshes.add(shape::UVSphere::default().into()),
        material: debug_material,
        transform: Transform::from_xyz(0.0, 1.0, 0.0),

        ..Default::default()
    });
}

fn update(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut PlayerTarget), With<Player>>,
    camera_query: Query<(&Camera, &mut GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_target) = query.single_mut();
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    // handle right click
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_global_transform, pos) {
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) {
                    let point = ray.get_point(distance);
                    crate::click::spawn(commands, meshes, materials, point, time.elapsed_seconds());
                    player_target.0 = Some(point);
                }
            }
        }
    }

    // move the player towards target
    if let Some(target) = player_target.0 {
        // Only move player by x and z
        let mut target = target.clone();
        target.y = player_transform.translation.y;

        let delta = target - player_transform.translation;

        if delta.length() == 0.0 {
            player_target.0 = None
        } else {
            player_transform.translation +=
                delta.clamp_length(0.0, time.delta_seconds() * PLAYER_SPEED);
        }
    }
}
