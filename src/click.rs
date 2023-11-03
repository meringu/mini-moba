use std::f32::consts::PI;

use bevy::prelude::*;

const CLICK_ANIMATION_SECONDS: f32 = 1.0;

pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update)
            .add_event::<ClickEvent>();
    }
}

#[derive(Event)]
pub struct ClickEvent(pub Vec3);

#[derive(Component, Default)]
struct Click;

#[derive(Component, Default)]
struct ClickSpawnTime(f32);

#[derive(Bundle, Default)]
struct ClickBundle {
    marker: Click,
    spawn_time: ClickSpawnTime,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

#[derive(Resource)]
struct ClickAssets {
    mesh: Handle<Mesh>,
    mat: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ClickAssets {
        mesh: meshes.add(shape::Circle::default().into()),
        mat: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..Default::default()
        }),
    });
}

fn update(
    mut commands: Commands,
    click_assets: Res<ClickAssets>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &ClickSpawnTime), With<Click>>,
    mut ev_click: EventReader<ClickEvent>,
) {
    // spawn new clicks
    for ev in ev_click.iter() {
        let location = ev.0;
        let mut transform = Transform::from_xyz(location.x, 0.1, location.z); // render just above the map
        transform.rotate_x(PI * 1.5);

        commands.spawn(ClickBundle {
            marker: Click,
            transform,
            spawn_time: ClickSpawnTime(time.elapsed_seconds()),
            mesh: click_assets.mesh.clone(),
            material: click_assets.mat.clone(),

            ..Default::default()
        });
    }

    // shrink clicks
    for (entity, mut transform, spawn_time) in query.iter_mut() {
        let size = (spawn_time.0 + CLICK_ANIMATION_SECONDS - time.elapsed_seconds())
            / CLICK_ANIMATION_SECONDS;

        if size < 0. {
            commands.entity(entity).despawn();
        } else {
            transform.scale.x = size;
            transform.scale.y = size;
            transform.scale.z = size;
        }
    }
}
