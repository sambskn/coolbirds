use bevy::prelude::*;

use crate::bird::{BirdGenInputs, generate_bird_body_mesh, generate_bird_head_mesh};

mod bird;

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
        .insert_resource(ClearColor(Color::srgb(0.35, 0.69, 0.36)))
        .add_systems(Startup, (spawn_camera_and_light, spawn_debug_mesh))
        .add_systems(Update, camera_update)
        .run();
}

fn spawn_debug_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let basic_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.83, 0.26, 0.17),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(generate_bird_head_mesh(BirdGenInputs::default()))),
        MeshMaterial3d(basic_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(generate_bird_body_mesh(BirdGenInputs::default()))),
        MeshMaterial3d(basic_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn spawn_camera_and_light(mut commands: Commands) {
    // spawn camera
    commands.spawn((
        Camera3d { ..default() },
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

const CAM_SENSITIVITY_X: f32 = 1.1;
const CAM_SENSITIVITY_Y: f32 = 0.7;
const SPEED: f32 = 12.0;

fn camera_update(
    camera_transform: Query<&mut Transform, With<Camera3d>>,
    gamepads: Query<&Gamepad>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    for mut cam in camera_transform {
        for gamepad in gamepads {
            // get unscaled stick inputs + dpad
            let l_stick = gamepad.left_stick();
            let r_stick = gamepad.right_stick();
            let d_pad = gamepad.dpad();
            // also keyboard i guess *eyeroll*
            let kb_wasd = Vec2::new(
                if keyboard_input.pressed(KeyCode::KeyD) {
                    1.0
                } else if keyboard_input.pressed(KeyCode::KeyA) {
                    -1.0
                } else {
                    0.0
                },
                if keyboard_input.pressed(KeyCode::KeyW) {
                    1.0
                } else if keyboard_input.pressed(KeyCode::KeyS) {
                    -1.0
                } else {
                    0.0
                },
            );

            // movement
            let combined_stick_magnitude = l_stick.length() + d_pad.length() + kb_wasd.length();
            if combined_stick_magnitude > 0.1 {
                let combined_movement_intent = (l_stick + d_pad + kb_wasd).normalize();
                let move_vec = combined_movement_intent * SPEED * timer.delta_secs();

                let offset = move_vec.x * cam.local_x() + move_vec.y * -1.0 * cam.local_z();
                cam.translation += offset;
            }

            // camera
            if r_stick.length() > 0.1 {
                let mut cam_adjust = r_stick;
                cam_adjust.x *= CAM_SENSITIVITY_X;
                cam_adjust.y *= CAM_SENSITIVITY_Y;
                cam.rotate_y(-1.0 * cam_adjust.x * timer.delta_secs());
                cam.rotate_local_x(cam_adjust.y * timer.delta_secs());
            }

            // debug prints
            if gamepad.just_pressed(GamepadButton::South) {
                // print out current camera tf
                info!("camera tf {:?}", cam);
            }
        }
    }
}
