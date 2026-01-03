use crate::{
    bird::{BirdGenInputs, RecentBirds, generate_bird_body_mesh, generate_bird_head_mesh},
    log_text::{LogTextPlugin, NewLog},
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
use rand::seq::IndexedRandom;

mod bird;
mod log_text;
mod ui;

pub const BG_COLOR: Color = Color::srgb(0.47, 0.49, 0.68);

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
        .add_plugins(LogTextPlugin)
        .add_plugins(InputDispatchPlugin)
        .add_plugins(TabNavigationPlugin)
        .add_message::<RebuildBird>()
        .insert_state(BirdState::BirdVisible)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(BirdGenInputs::default())
        .insert_resource(RecentBirds {
            left: BirdGenInputs::default(),
            right: BirdGenInputs::default(),
        })
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
    mut log_writer: MessageWriter<NewLog>,
) {
    for _event in bird_rebuild_reader.read() {
        log_writer.write(NewLog {
            text: "loading fresh birds".to_string(),
        });
        for bird_mesh_entity in bird_mesh_query.iter() {
            commands.entity(bird_mesh_entity).despawn();
        }
        next_bird_state.set(BirdState::Loading);
    }
}

#[derive(Component)]
struct BirdMesh;

#[derive(Component)]
struct BirdCam {
    pub focus: BirdCamFocus,
}

#[derive(Copy, Clone)]
enum BirdCamFocus {
    LeftBird,
    RightBird,
    SeedBird,
}

impl BirdCam {
    pub fn left() -> Self {
        BirdCam {
            focus: BirdCamFocus::LeftBird,
        }
    }
    pub fn right() -> Self {
        BirdCam {
            focus: BirdCamFocus::RightBird,
        }
    }
    pub fn seed() -> Self {
        BirdCam {
            focus: BirdCamFocus::SeedBird,
        }
    }
}

fn get_bird_transform_offset(focus: BirdCamFocus) -> Vec3 {
    let mult = match focus {
        BirdCamFocus::LeftBird => 0.0,
        BirdCamFocus::RightBird => 1.0,
        BirdCamFocus::SeedBird => -1.0,
    };
    Vec3 {
        x: 0.0,
        y: 100000.0 * mult,
        z: 0.0,
    }
}

fn spawn_bird_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_bird_state: ResMut<NextState<BirdState>>,
    mut recent_birds: ResMut<RecentBirds>,
    bird_inputs: Res<BirdGenInputs>,
) {
    let colors = get_colors(3);
    let left_bird_mat = materials.add(StandardMaterial {
        base_color: colors[0],
        ..default()
    });
    let right_bird_mat = materials.add(StandardMaterial {
        base_color: colors[1],
        ..default()
    });
    let seed_bird_mat = materials.add(StandardMaterial {
        base_color: colors[2],
        ..default()
    });

    let current_bird_inputs = bird_inputs.into_inner();
    // create mesh of current bird for display
    let seed_head_mesh = generate_bird_head_mesh(&current_bird_inputs);
    let seed_body_mesh = generate_bird_body_mesh(&current_bird_inputs);

    let mut random_bird_inputs = BirdGenInputs::default();
    random_bird_inputs.randomize_values();
    let left_bird_inputs = current_bird_inputs.get_child_with(&random_bird_inputs);
    let right_bird_inputs = current_bird_inputs.get_child_with(&random_bird_inputs);

    // update RecentBirds
    recent_birds.left = left_bird_inputs;
    recent_birds.right = right_bird_inputs;

    let left_head_mesh = generate_bird_head_mesh(&left_bird_inputs);
    let left_body_mesh = generate_bird_body_mesh(&left_bird_inputs);
    let right_head_mesh = generate_bird_head_mesh(&right_bird_inputs);
    let right_body_mesh = generate_bird_body_mesh(&right_bird_inputs);
    commands.spawn((
        Mesh3d(meshes.add(seed_head_mesh)),
        MeshMaterial3d(seed_bird_mat.clone()),
        Transform::from_xyz(0.0, -100000.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(seed_body_mesh)),
        MeshMaterial3d(seed_bird_mat),
        Transform::from_xyz(0.0, -100000.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(left_head_mesh)),
        MeshMaterial3d(left_bird_mat.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(left_body_mesh)),
        MeshMaterial3d(left_bird_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(right_head_mesh)),
        MeshMaterial3d(right_bird_mat.clone()),
        Transform::from_xyz(0.0, 100000.0, 0.0),
        BirdMesh,
    ));
    commands.spawn((
        Mesh3d(meshes.add(right_body_mesh)),
        MeshMaterial3d(right_bird_mat),
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
    let buffer = get_bird_camera_buffer_size_from_window(window_size);
    let bird_box_size = get_bird_camera_size_from_window(window_size, buffer);
    let bird_seed_preview_size = bird_box_size / 2;
    let centered_y = (window_size.y - bird_box_size) / 2;
    let centered_x = (window_size.x - bird_box_size / 2) / 2;
    // Position camera to look at origin
    let cam_offset = 80.0;
    let camera_pos = Vec3::new(cam_offset, cam_offset, cam_offset);
    let look_at = Vec3::ZERO;

    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: window_size.x / 2 - bird_box_size - buffer,
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
        BirdCam::left(),
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: window_size.x / 2 + buffer,
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
        BirdCam::right(),
        Transform::from_translation(
            camera_pos + get_bird_transform_offset(BirdCamFocus::RightBird),
        )
        .looking_at(
            look_at + get_bird_transform_offset(BirdCamFocus::RightBird),
            Vec3::Y,
        ),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: centered_x,
                    y: centered_y - bird_seed_preview_size,
                },
                physical_size: UVec2 {
                    x: bird_seed_preview_size,
                    y: bird_seed_preview_size,
                },
                ..default()
            }),
            order: 2,
            ..default()
        },
        BirdCam::seed(),
        Transform::from_translation(camera_pos + get_bird_transform_offset(BirdCamFocus::SeedBird))
            .looking_at(
                look_at + get_bird_transform_offset(BirdCamFocus::SeedBird),
                Vec3::Y,
            ),
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

        let origin = Vec3::ZERO + get_bird_transform_offset(bird_cam.focus);

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
        let buffer = get_bird_camera_buffer_size_from_window(window_size);
        let bird_box_size = get_bird_camera_size_from_window(window_size, buffer);
        let bird_seed_preview_size = bird_box_size / 2;
        let centered_y = (window_size.y - bird_box_size) / 2;
        let centered_x = (window_size.x - bird_box_size / 2) / 2;
        match bird_cam.focus {
            BirdCamFocus::LeftBird => {
                viewport.physical_size = UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                };
                viewport.physical_position = UVec2 {
                    x: window_size.x / 2 - bird_box_size - buffer,
                    y: centered_y,
                };
            }
            BirdCamFocus::RightBird => {
                viewport.physical_size = UVec2 {
                    x: bird_box_size,
                    y: bird_box_size,
                };
                viewport.physical_position = UVec2 {
                    x: window_size.x / 2 + buffer,
                    y: centered_y,
                };
            }
            BirdCamFocus::SeedBird => {
                viewport.physical_position = UVec2 {
                    x: centered_x,
                    y: centered_y - bird_seed_preview_size,
                };
                viewport.physical_size = UVec2 {
                    x: bird_seed_preview_size,
                    y: bird_seed_preview_size,
                };
            }
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
                let origin = Vec3::ZERO + get_bird_transform_offset(bird_cam.focus);
                let direction = (tf.translation - origin).normalize();
                let current_distance = tf.translation.distance(origin);

                let new_distance =
                    (current_distance - zoom_delta).clamp(ZOOM_MIN_DISTANCE, ZOOM_MAX_DISTANCE);
                tf.translation = origin + direction * new_distance;
            }
        }
    }
}

fn get_colors(num_colors: usize) -> Vec<Color> {
    let mut rng = rand::rng();
    let sources = vec![
        Color::linear_rgb(247.0 / 255.0, 37.0 / 255.0, 133.0 / 255.0), // Neon Pink
        Color::linear_rgb(49.0 / 255.0, 55.0 / 255.0, 21.0 / 255.0),   // Dark Khaki
        Color::linear_rgb(58.0 / 255.0, 12.0 / 255.0, 163.0 / 255.0),  // Vivid Royal
        Color::linear_rgb(190.0 / 255.0, 237.0 / 255.0, 170.0 / 255.0), // Tea Green
        Color::linear_rgb(247.0 / 255.0, 179.0 / 255.0, 43.0 / 255.0), // Sunflower Gold
    ];
    sources
        .choose_multiple(&mut rng, num_colors)
        .map(|color| *color)
        .collect()
}

// these two heleprs are used to make sure our bird viewports dont get too big
fn get_bird_camera_size_from_window(window_size: UVec2, buffer: u32) -> u32 {
    (window_size.y / 2).min(window_size.x / 2) - buffer
}

fn get_bird_camera_buffer_size_from_window(window_size: UVec2) -> u32 {
    (window_size.y / 32).min(window_size.x / 32)
}
