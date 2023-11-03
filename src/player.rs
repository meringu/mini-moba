use bevy::prelude::*;

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 10.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update)
            .add_event::<PlayerCommand>();
    }
}

#[derive(Component, Default)]
struct Player {
    state: PlayerState,
}

#[derive(Event)]
pub enum PlayerCommand {
    Move(Vec3),
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
    time: Res<Time>,
    mut ev_player: EventReader<PlayerCommand>,
) {
    let (mut player_transform, mut player) = query.single_mut();

    for ev in ev_player.iter() {
        match ev {
            PlayerCommand::Move(location) => player.state = PlayerState::Moving(*location),
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
