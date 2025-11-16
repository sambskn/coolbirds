use crate::bird::{BirdGenInputs, generate_bird_body_mesh, generate_bird_head_mesh};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod bird;

const BG_COLOR: Color = Color::srgb(0.47, 0.69, 0.48);

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
        .add_plugins(EguiPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_message::<RebuildBird>()
        .insert_state(BirdState::Loading)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(BirdGenInputs::default())
        .add_systems(Startup, (spawn_camera_and_light, spawn_bird_mesh))
        .add_systems(Update, handle_bird_rebuild)
        .add_systems(OnEnter(BirdState::Loading), spawn_bird_mesh)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut bird_inputs: ResMut<BirdGenInputs>,
    mut remake_the_bird: MessageWriter<RebuildBird>,
    mut next_bird_state: ResMut<NextState<BirdState>>,
    bird_state: Res<State<BirdState>>,
) -> Result {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(contexts.ctx_mut().unwrap(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Beak");
                ui.add(
                    egui::Slider::new(&mut bird_inputs.beak_length, 0.0..=50.0).text("Beak Length"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.beak_size, 20.0..=100.0).text("Beak Size"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.beak_width, 0.0..=25.0).text("Beak Width"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.beak_roundness, 10.0..=200.0)
                        .text("Beak Roundness"),
                );

                ui.separator();
                ui.heading("Head");
                ui.add(
                    egui::Slider::new(&mut bird_inputs.head_size, 10.0..=40.0).text("Head Size"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.head_to_belly, -20.0..=50.0)
                        .text("Head to Belly"),
                );
                ui.add(egui::Slider::new(&mut bird_inputs.eye_size, 0.0..=20.0).text("Eye Size"));
                ui.add(
                    egui::Slider::new(&mut bird_inputs.head_lateral_offset, -15.0..=15.0)
                        .text("Head Lateral Offset"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.head_level, 0.0..=80.0).text("Head Level"),
                );
                ui.add(egui::Slider::new(&mut bird_inputs.head_yaw, -45.0..=45.0).text("Head Yaw"));
                ui.add(
                    egui::Slider::new(&mut bird_inputs.head_pitch, -80.0..=45.0).text("Head Pitch"),
                );

                ui.separator();
                ui.heading("Body");
                ui.add(
                    egui::Slider::new(&mut bird_inputs.belly_length, 10.0..=100.0)
                        .text("Belly Length"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.belly_size, 20.0..=60.0).text("Belly Size"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.belly_fat, 50.0..=150.0).text("Belly Fat"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.belly_to_bottom, 1.0..=50.0)
                        .text("Belly to Bottom"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.bottom_size, 5.0..=50.0).text("Bottom Size"),
                );

                ui.separator();
                ui.heading("Tail");
                ui.add(
                    egui::Slider::new(&mut bird_inputs.tail_length, 0.0..=100.0)
                        .text("Tail Length"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.tail_width, 1.0..=50.0).text("Tail Width"),
                );
                ui.add(egui::Slider::new(&mut bird_inputs.tail_yaw, -45.0..=45.0).text("Tail Yaw"));
                ui.add(
                    egui::Slider::new(&mut bird_inputs.tail_pitch, -45.0..=90.0).text("Tail Pitch"),
                );
                ui.add(
                    egui::Slider::new(&mut bird_inputs.tail_roundness, 10.0..=200.0)
                        .text("Tail Roundness"),
                );

                // Not actually implemented right now lol
                // ui.separator();
                // ui.heading("Base");
                // ui.add(
                //     egui::Slider::new(&mut bird_inputs.base_flat, -100.0..=100.0).text("Base Flat"),
                // );

                ui.separator();
                if *bird_state.get() == BirdState::BirdVisible {
                    if ui.button("regenerate bird").clicked() {
                        remake_the_bird.write(RebuildBird);
                        next_bird_state.set(BirdState::Loading);
                    }
                } else {
                    ui.label("generating bird...");
                }
                ui.separator();
                ui.label("ported/inspired by bird-o-matic by mooncactus");
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    Ok(())
}

fn handle_bird_rebuild(
    mut bird_rebuild_message: MessageReader<RebuildBird>,
    bird_mesh_query: Query<Entity, With<BirdMesh>>,
    mut commands: Commands,
) {
    for _bird_rebuild_event in bird_rebuild_message.read() {
        // kill all bird meshes
        for brid_mesh_entity in bird_mesh_query {
            commands.entity(brid_mesh_entity).despawn();
        }
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
    // spawn camera
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(65.0, 40.0, 65.0).with_rotation(quat(
            -0.07382465,
            0.46779895,
            0.039250545,
            0.8798623,
        )),
    ));

    // spawn lighting
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
