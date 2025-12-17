use crate::{
    bird::{BirdGenInputs, generate_bird_body_mesh, generate_bird_head_mesh},
    ui::BirdUIPlugin,
};
use bevy::{
    camera::Viewport,
    input::{
        ButtonInput,
        mouse::{MouseMotion, MouseWheel},
        touch::Touch,
    },
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    picking::hover::HoverMap,
    prelude::*,
};
use bevy_mod_clipboard::ClipboardPlugin;

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
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "bird-o-matic".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
            ClipboardPlugin,
        ))
        .add_plugins(InputDispatchPlugin)
        .add_plugins(TabNavigationPlugin)
        .add_message::<RebuildBird>()
        .insert_state(BirdState::BirdVisible)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(BirdGenInputs::default())
        .add_plugins(BirdUIPlugin)
        .add_systems(Startup, (spawn_camera_and_light, kick_off_bird_load))
        .add_systems(
            Update,
            (
                handle_bird_rebuild,
                touch_system,
                mouse_drag_system,
                zoom_system,
            ),
        )
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

#[derive(Component)]
struct BirdCam {
    pub index: usize,
}

fn bird_offset_for_index(index: usize) -> Vec3 {
    Vec3 {
        x: 0.0,
        y: 100000.0 * index as f32,
        z: 0.0,
    }
}

fn spawn_bird_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_bird_state: ResMut<NextState<BirdState>>,
    bird_inputs: Res<BirdGenInputs>,
) {
    info!("time to spawn bird");
    let colors = get_two_colors();
    let basic_material = materials.add(StandardMaterial {
        base_color: colors[0],
        ..default()
    });
    let basic_material_2 = materials.add(StandardMaterial {
        base_color: colors[1],
        ..default()
    });
    let current_bird_inputs = bird_inputs.into_inner();
    let mut random_bird_inputs = BirdGenInputs::default();
    random_bird_inputs.randomize_values();
    let left_bird_inputs = current_bird_inputs.get_child_with(&random_bird_inputs);
    let right_bird_inputs = current_bird_inputs.get_child_with(&random_bird_inputs);

    let left_head_mesh = generate_bird_head_mesh(&left_bird_inputs);
    let left_body_mesh = generate_bird_body_mesh(&left_bird_inputs);
    let right_head_mesh = generate_bird_head_mesh(&right_bird_inputs);
    let right_body_mesh = generate_bird_body_mesh(&right_bird_inputs);

    commands.spawn((
        Mesh3d(meshes.add(left_head_mesh)),
        MeshMaterial3d(basic_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(left_body_mesh)),
        MeshMaterial3d(basic_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(right_head_mesh)),
        MeshMaterial3d(basic_material_2.clone()),
        Transform::from_xyz(0.0, 100000.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(right_body_mesh)),
        MeshMaterial3d(basic_material_2),
        Transform::from_xyz(0.0, 100000.0, 0.0),
        BirdMesh,
    ));
    next_bird_state.set(BirdState::BirdVisible);
}

fn spawn_camera_and_light(
    mut commands: Commands,
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let window_size = window.physical_size();
    let bird_box_size = window_size.y / 2 - 4;
    let centered_y = (window_size.y - bird_box_size) / 2;
    // Position camera to look at origin
    let camera_pos = Vec3::new(65.0, 40.0, 65.0);
    let look_at = Vec3::ZERO;

    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: window_size.x / 2 - bird_box_size - 4,
                    y: centered_y,
                },
                physical_size: UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                },
                ..default()
            }),
            order: 0,
            ..default()
        },
        BirdCam { index: 0 },
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: window_size.x / 2 + 4,
                    y: centered_y,
                },
                physical_size: UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                },
                ..default()
            }),
            order: 1,
            ..default()
        },
        BirdCam { index: 1 },
        Transform::from_translation(camera_pos + bird_offset_for_index(1))
            .looking_at(look_at + bird_offset_for_index(1), Vec3::Y),
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

const TOUCH_ADJUST_SPEED: f32 = 0.05;

fn touch_system(
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
    touches: Res<Touches>,
    mut cam_query: Query<(&mut Transform, Entity, &mut Camera, &BirdCam), With<Camera3d>>,
    time: Res<Time>,
    hovermap: Res<HoverMap>,
) {
    let mut rotate_intent = Vec2::ZERO;
    for touch in touches.iter() {
        rotate_intent += touch.position() - touch.previous_position()
    }

    if rotate_intent.length() > 0.05 {
        let window_size = window.physical_size();
        apply_rotation(
            &mut cam_query,
            &hovermap,
            rotate_intent,
            TOUCH_ADJUST_SPEED * time.delta_secs(),
            window_size,
        );
    }
}

const MOUSE_ADJUST_SPEED: f32 = 0.002;

fn mouse_drag_system(
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut cam_query: Query<(&mut Transform, Entity, &mut Camera, &BirdCam), With<Camera3d>>,
    hovermap: Res<HoverMap>,
) {
    // Only rotate when left mouse button is held
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    let mut rotate_intent = Vec2::ZERO;
    for motion in mouse_motion.read() {
        rotate_intent += motion.delta;
    }

    if rotate_intent.length() > 0.05 {
        let window_size = window.physical_size();
        apply_rotation(
            &mut cam_query,
            &hovermap,
            rotate_intent,
            MOUSE_ADJUST_SPEED,
            window_size,
        );
    }
}

fn apply_rotation(
    cam_query: &mut Query<(&mut Transform, Entity, &mut Camera, &BirdCam), With<Camera3d>>,
    hovermap: &Res<HoverMap>,
    rotate_intent: Vec2,
    speed_multiplier: f32,
    window_size: UVec2,
) {
    for (mut tf, entity, mut cam, bird_cam) in cam_query.iter_mut() {
        let cam_match = check_if_hovering_not_ui(&hovermap, entity);
        if cam_match {
            continue;
        }

        let origin = Vec3::ZERO + bird_offset_for_index(bird_cam.index);

        // X motion = orbit around Y axis (yaw)
        let yaw_delta = rotate_intent.x * std::f32::consts::PI * speed_multiplier;

        // Rotate position around the Y axis
        let rotation = Quat::from_rotation_y(yaw_delta);
        let offset = tf.translation - origin;
        tf.translation = origin + rotation * offset;

        // Y motion = adjust pitch (tilt up/down)
        let pitch_delta = -rotate_intent.y * std::f32::consts::PI * speed_multiplier;

        // Calculate current pitch and clamp it
        let forward = (origin - tf.translation).normalize();
        let current_pitch = forward.y.asin();
        let new_pitch = (current_pitch + pitch_delta).clamp(
            -std::f32::consts::FRAC_PI_2 * 0.95,
            std::f32::consts::FRAC_PI_2 * 0.95,
        );
        let actual_pitch_delta = new_pitch - current_pitch;

        // Rotate around the right axis for pitch
        let right = tf.right();
        let pitch_rotation = Quat::from_axis_angle(*right, actual_pitch_delta);
        let offset = tf.translation - origin;
        tf.translation = origin + pitch_rotation * offset;

        // Apply rotation to keep looking at origin
        tf.look_at(origin, Vec3::Y);

        // Ensure camera viewport is how we want
        let Some(viewport) = &mut cam.viewport else {
            continue;
        };
        // Make any changes to viewport
        let bird_box_size = window_size.y / 2 - 4;
        let centered_y = (window_size.y - bird_box_size) / 2;
        match bird_cam.index {
            0 => {
                viewport.physical_size = UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                };
                viewport.physical_position = UVec2 {
                    x: window_size.x / 2 - bird_box_size,
                    y: centered_y,
                };
            }
            1 => {
                viewport.physical_size = UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                };
                viewport.physical_position = UVec2 {
                    x: window_size.x / 2 + 4,
                    y: centered_y,
                };
            }
            _ => {}
        }
    }
}

fn check_if_hovering_not_ui(hovermap: &Res<HoverMap>, camera_entity: Entity) -> bool {
    // Check if we're hovering over actual content for this 3d camera (not the UI)
    let mut cam_match = false;
    for hovers in hovermap.0.values() {
        for hoverhit in hovers.values() {
            if hoverhit.camera == camera_entity {
                cam_match = true;
            }
        }
    }
    return cam_match;
}

const ZOOM_SPEED: f32 = 0.1;
const ZOOM_MIN_DISTANCE: f32 = 2.0;
const ZOOM_MAX_DISTANCE: f32 = 400.0;

fn zoom_system(
    mut mouse_wheel: MessageReader<MouseWheel>,
    touches: Res<Touches>,
    mut cam_query: Query<(&mut Transform, Entity, &BirdCam), With<Camera3d>>,
    mut previous_pinch_distance: Local<Option<f32>>,
    hovermap: Res<HoverMap>,
) {
    let mut zoom_delta = 0.0;

    // Handle mouse wheel
    for event in mouse_wheel.read() {
        zoom_delta += event.y * ZOOM_SPEED;
    }

    // Handle pinch gestures
    if touches.iter().count() == 2 {
        let touch_vec: Vec<&Touch> = touches.iter().collect();
        let touch1_pos = touch_vec[0].position();
        let touch2_pos = touch_vec[1].position();

        let current_distance = touch1_pos.distance(touch2_pos);

        if let Some(prev_distance) = *previous_pinch_distance {
            // Pinch in = zoom out, pinch out = zoom in
            let pinch_delta = (prev_distance - current_distance) * 0.01;
            zoom_delta -= pinch_delta;
        }

        *previous_pinch_distance = Some(current_distance);
    } else {
        *previous_pinch_distance = None;
    }

    // Apply zoom
    for (mut tf, ent, bird_cam) in cam_query.iter_mut() {
        if zoom_delta.abs() > 0.001 {
            // Check if we're hovering over actual content for this 3d camera (not the UI)
            let cam_match = check_if_hovering_not_ui(&hovermap, ent);
            // not exactly sure why im inverting this but its what works lol
            if !cam_match {
                let origin = Vec3::ZERO + bird_offset_for_index(bird_cam.index);
                let direction = (tf.translation - origin).normalize();
                let current_distance = tf.translation.distance(origin);

                let new_distance =
                    (current_distance - zoom_delta).clamp(ZOOM_MIN_DISTANCE, ZOOM_MAX_DISTANCE);
                tf.translation = origin + direction * new_distance;
            }
        }
    }
}

fn get_two_colors() -> [Color; 2] {
    use rand::Rng;
    let mut rng = rand::rng();
    let sources = vec![
        Color::linear_rgb(247.0 / 255.0, 37.0 / 255.0, 133.0 / 255.0), // Neon Pink
        Color::linear_rgb(49.0 / 255.0, 55.0 / 255.0, 21.0 / 255.0),   // Dark Khaki
        Color::linear_rgb(58.0 / 255.0, 12.0 / 255.0, 163.0 / 255.0),  // Vivid Royal
        Color::linear_rgb(190.0 / 255.0, 237.0 / 255.0, 170.0 / 255.0), // Tea Green
        Color::linear_rgb(247.0 / 255.0, 179.0 / 255.0, 43.0 / 255.0), // Sunflower Gold
    ];
    let idx = rng.random_range(0..=4) as usize;
    [sources[idx], sources[if idx == 0 { 4 } else { idx - 1 }]]
}
