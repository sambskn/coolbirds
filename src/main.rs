use crate::{
    bird::{BirdGenInputs, generate_bird_body_mesh, generate_bird_head_mesh},
    ui::BirdUIPlugin,
};
use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
};

mod bird;
mod ui;

const BG_COLOR: Color = Color::srgb(0.47, 0.49, 0.68);

#[derive(Message, Debug)]
struct RebuildBird;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum BirdState {
    Loading,
    BirdVisible,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "rusty-bird".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .add_plugins(InputDispatchPlugin)
        .add_plugins(TabNavigationPlugin)
        .add_message::<RebuildBird>()
        .insert_state(BirdState::BirdVisible)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(BirdGenInputs::default())
        .add_plugins(BirdUIPlugin)
        .add_systems(Startup, (spawn_camera_and_light, kick_off_bird_load))
        .add_systems(Update, handle_bird_rebuild)
        .add_systems(OnEnter(BirdState::Loading), spawn_bird_mesh)
        .run();
}

fn kick_off_bird_load(mut next_bird_state: ResMut<NextState<BirdState>>) {
    next_bird_state.set(BirdState::Loading);
}
fn handle_bird_rebuild(
    mut bird_rebuild_reader: MessageReader<RebuildBird>,
    bird_mesh_query: Query<Entity, With<BirdMesh>>,
    mut commands: Commands,
    mut next_bird_state: ResMut<NextState<BirdState>>,
) {
    for _event in bird_rebuild_reader.read() {
        for bird_mesh_entity in bird_mesh_query.iter() {
            info!("bird mesh kill");
            commands.entity(bird_mesh_entity).despawn();
        }
        next_bird_state.set(BirdState::Loading);
    }
}

#[derive(Component)]
struct BirdMesh;

fn spawn_bird_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_bird_state: ResMut<NextState<BirdState>>,
    bird_inputs: Res<BirdGenInputs>,
) {
    info!("time to spawn bird");
    let basic_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.83, 0.26, 0.17),
        ..default()
    });
    let current_bird_inputs = bird_inputs.into_inner();
    commands.spawn((
        Mesh3d(meshes.add(generate_bird_head_mesh(current_bird_inputs))),
        MeshMaterial3d(basic_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(generate_bird_body_mesh(current_bird_inputs))),
        MeshMaterial3d(basic_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    next_bird_state.set(BirdState::BirdVisible);
}

fn spawn_camera_and_light(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(65.0, 40.0, 65.0).with_rotation(Quat::from_xyzw(
            -0.07382465,
            0.46779895,
            0.039250545,
            0.8798623,
        )),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 30000.0,
            ..default()
        },
        Transform::from_xyz(25.0, 20.0, 10.0).with_rotation(Quat::from_xyzw(
            -0.2638357, 0.52681506, 0.1762679, 0.7885283,
        )),
    ));
}
