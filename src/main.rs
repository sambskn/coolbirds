use crate::bird::{
    BirdGenInputTypes, BirdGenInputs, generate_bird_body_mesh, generate_bird_head_mesh,
    get_input_type_string, get_input_value_for_type,
};
use bevy::{
    color::palettes::basic::*,
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabGroup, TabNavigationPlugin},
    },
    picking::hover::Hovered,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{
        Activate, Button, CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue,
        TrackClick, UiWidgetsPlugins, ValueChange, observe,
    },
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod bird;

const BG_COLOR: Color = Color::srgb(0.47, 0.69, 0.48);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Message, Debug)]
struct RebuildBird;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum BirdState {
    Loading,
    BirdVisible,
}

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct RegenerateButton;

#[derive(Component)]
struct BirdInputSlider {
    input_type: BirdGenInputTypes,
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
        .add_plugins(UiWidgetsPlugins)
        .add_plugins(InputDispatchPlugin)
        .add_plugins(TabNavigationPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_message::<RebuildBird>()
        .insert_state(BirdState::BirdVisible)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(BirdGenInputs::default())
        .add_systems(
            Startup,
            (spawn_camera_and_light, kick_off_bird_load, setup_ui),
        )
        .add_systems(
            Update,
            (
                update_slider_values,
                handle_bird_rebuild,
                update_slider_styles,
                update_slider_styles2,
                update_button_style,
                update_button_style2,
            ),
        )
        .add_systems(OnEnter(BirdState::Loading), spawn_bird_mesh)
        .run();
}

fn kick_off_bird_load(mut next_bird_state: ResMut<NextState<BirdState>>) {
    next_bird_state.set(BirdState::Loading);
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ui_root(&asset_server));
}

fn ui_root(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            top: Val::Px(0.),
            width: Val::Px(300.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
        TabGroup::default(),
        UiRoot,
        children![
            // Beak Section
            section_header(asset_server, "Beak"),
            slider(
                asset_server,
                |inputs, v| inputs.beak_length = v,
                BirdGenInputTypes::BeakLength,
                0.0,
                50.0,
                25.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.beak_size = v,
                BirdGenInputTypes::BeakSize,
                20.0,
                100.0,
                60.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.beak_width = v,
                BirdGenInputTypes::BeakWidth,
                0.0,
                25.0,
                12.5
            ),
            slider(
                asset_server,
                |inputs, v| inputs.beak_roundness = v,
                BirdGenInputTypes::BeakRoundness,
                10.0,
                200.0,
                105.0
            ),
            separator(),
            // Head Section
            section_header(asset_server, "Head"),
            slider(
                asset_server,
                |inputs, v| inputs.head_size = v,
                BirdGenInputTypes::HeadSize,
                10.0,
                40.0,
                25.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.head_to_belly = v,
                BirdGenInputTypes::HeadToBelly,
                -20.0,
                50.0,
                15.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.eye_size = v,
                BirdGenInputTypes::EyeSize,
                0.0,
                20.0,
                10.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.head_lateral_offset = v,
                BirdGenInputTypes::HeadLateralOffset,
                -15.0,
                15.0,
                0.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.head_level = v,
                BirdGenInputTypes::HeadLevel,
                0.0,
                80.0,
                40.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.head_yaw = v,
                BirdGenInputTypes::HeadYaw,
                -45.0,
                45.0,
                0.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.head_pitch = v,
                BirdGenInputTypes::HeadPitch,
                -80.0,
                45.0,
                -17.5
            ),
            separator(),
            // Body Section
            section_header(asset_server, "Body"),
            slider(
                asset_server,
                |inputs, v| inputs.belly_length = v,
                BirdGenInputTypes::BellyLength,
                10.0,
                100.0,
                55.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.belly_size = v,
                BirdGenInputTypes::BellySize,
                20.0,
                60.0,
                40.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.belly_fat = v,
                BirdGenInputTypes::BellyFat,
                50.0,
                150.0,
                100.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.belly_to_bottom = v,
                BirdGenInputTypes::BellyToBottom,
                1.0,
                50.0,
                25.5
            ),
            slider(
                asset_server,
                |inputs, v| inputs.bottom_size = v,
                BirdGenInputTypes::BottomSize,
                5.0,
                50.0,
                27.5
            ),
            separator(),
            // Tail Section
            section_header(asset_server, "Tail"),
            slider(
                asset_server,
                |inputs, v| inputs.tail_length = v,
                BirdGenInputTypes::TailLength,
                0.0,
                100.0,
                50.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.tail_width = v,
                BirdGenInputTypes::TailWidth,
                1.0,
                50.0,
                25.5
            ),
            slider(
                asset_server,
                |inputs, v| inputs.tail_yaw = v,
                BirdGenInputTypes::TailYaw,
                -45.0,
                45.0,
                0.0
            ),
            slider(
                asset_server,
                |inputs, v| inputs.tail_pitch = v,
                BirdGenInputTypes::TailPitch,
                -45.0,
                90.0,
                22.5
            ),
            slider(
                asset_server,
                |inputs, v| inputs.tail_roundness = v,
                BirdGenInputTypes::TailRoundness,
                10.0,
                200.0,
                105.0
            ),
            separator(),
            // Regenerate Button
            (
                regenerate_button(asset_server),
                observe(
                    |_activate: On<Activate>,
                     mut rebuild_writer: MessageWriter<RebuildBird>,
                     bird_state: Res<State<BirdState>>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            rebuild_writer.write(RebuildBird);
                        }
                    }
                ),
            ),
            separator(),
            // Footer
            footer(asset_server),
        ],
    )
}

fn section_header(asset_server: &AssetServer, title: &str) -> impl Bundle {
    (
        Text::new(title),
        TextFont {
            font: asset_server.load("fonts/OTBrut-Regular.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Node {
            margin: UiRect::top(Val::Px(10.)),
            ..default()
        },
    )
}

fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(1.),
            margin: UiRect::vertical(Val::Px(10.)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
    )
}

fn slider<F>(
    asset_server: &AssetServer,
    update_fn: F,
    input_type: BirdGenInputTypes,
    min: f32,
    max: f32,
    value: f32,
) -> impl Bundle
where
    F: Fn(&mut BirdGenInputs, f32) + Send + Sync + 'static,
{
    (
        Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            margin: UiRect::vertical(Val::Px(5.)),
            ..default()
        },
        children![
            // Label
            (
                Text::new(get_input_type_string(&input_type)),
                TextFont {
                    font: asset_server.load("fonts/OTBrut-Regular.ttf"),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(3.)),
                    ..default()
                },
            ),
            // Slider
            (
                (
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Stretch,
                        height: Val::Px(12.),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    Hovered::default(),
                    BirdInputSlider { input_type },
                    Slider {
                        track_click: TrackClick::Snap,
                    },
                    SliderValue(value),
                    SliderRange::new(min, max),
                    children![
                        // Slider background rail
                        (
                            Node {
                                height: Val::Px(6.),
                                ..default()
                            },
                            BackgroundColor(SLIDER_TRACK),
                            BorderRadius::all(Val::Px(3.)),
                        ),
                        // Invisible track for thumb positioning
                        (
                            Node {
                                display: Display::Flex,
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.),
                                right: Val::Px(12.),
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                ..default()
                            },
                            children![
                                // Thumb
                                (
                                    SliderThumb,
                                    Node {
                                        display: Display::Flex,
                                        width: Val::Px(12.),
                                        height: Val::Px(12.),
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(0.),
                                        ..default()
                                    },
                                    BorderRadius::MAX,
                                    BackgroundColor(SLIDER_THUMB),
                                ),
                            ],
                        ),
                    ],
                ),
                observe(
                    move |value_change: On<ValueChange<f32>>,
                          mut bird_inputs: ResMut<BirdGenInputs>| {
                        update_fn(&mut bird_inputs, value_change.value);
                    }
                ),
            ),
        ],
    )
}

fn regenerate_button(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(40.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(5.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        Button,
        RegenerateButton,
        Hovered::default(),
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(Color::BLACK),
        BorderRadius::all(Val::Px(5.)),
        children![(
            Text::new("Regenerate Bird"),
            TextFont {
                font: asset_server.load("fonts/OTBrut-Regular.ttf"),
                font_size: 18.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}

fn footer(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new("ported/inspired by bird-o-matic by mooncactus"),
        TextFont {
            font: asset_server.load("fonts/OTBrut-Regular.ttf"),
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Node {
            margin: UiRect::top(Val::Px(10.)),
            ..default()
        },
    )
}

fn update_button_style(
    mut buttons: Query<
        (
            &Hovered,
            Has<InteractionDisabled>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (
            Or<(Changed<Hovered>, Added<InteractionDisabled>)>,
            With<RegenerateButton>,
        ),
    >,
) {
    for (hovered, disabled, mut color, mut border_color) in &mut buttons {
        set_button_style(disabled, hovered.get(), &mut color, &mut border_color);
    }
}

fn update_button_style2(
    mut buttons: Query<
        (
            &Hovered,
            Has<InteractionDisabled>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<RegenerateButton>,
    >,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_disabled.read().for_each(|entity| {
        if let Ok((hovered, disabled, mut color, mut border_color)) = buttons.get_mut(entity) {
            set_button_style(disabled, hovered.get(), &mut color, &mut border_color);
        }
    });
}

fn set_button_style(
    disabled: bool,
    hovered: bool,
    color: &mut BackgroundColor,
    border_color: &mut BorderColor,
) {
    match (disabled, hovered) {
        (true, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(GRAY);
        }
        (false, true) => {
            *color = HOVERED_BUTTON.into();
            border_color.set_all(WHITE);
        }
        (false, false) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(BLACK);
        }
    }
}

fn update_slider_styles(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<SliderRange>,
                Changed<Hovered>,
                Changed<CoreSliderDragState>,
                Added<InteractionDisabled>,
            )>,
            With<Slider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<SliderThumb>), Without<Slider>>,
) {
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                thumb_node.left = Val::Percent(range.thumb_position(value.0) * 100.0);
                thumb_bg.0 = thumb_color(disabled, hovered.0 || drag_state.dragging);
            }
        }
    }
}

fn update_slider_styles2(
    sliders: Query<
        (
            Entity,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        With<Slider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut BackgroundColor, Has<SliderThumb>), Without<Slider>>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_disabled.read().for_each(|entity| {
        if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity) {
            for child in children.iter_descendants(slider_ent) {
                if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                    && is_thumb
                {
                    thumb_bg.0 = thumb_color(disabled, hovered.0 || drag_state.dragging);
                }
            }
        }
    });
}

fn update_slider_values(
    res: Res<BirdGenInputs>,
    mut sliders: Query<(Entity, &mut Slider, &BirdInputSlider)>,
    mut commands: Commands,
) {
    if res.is_changed() {
        for (slider_ent, mut slider, bird_slider) in sliders.iter_mut() {
            commands
                .entity(slider_ent)
                .insert(SliderValue(get_input_value_for_type(
                    &bird_slider.input_type,
                    &res,
                )));
            // slider.track_click = res.slider_click;
        }
    }
}

fn thumb_color(disabled: bool, hovered: bool) -> Color {
    match (disabled, hovered) {
        (true, _) => Color::srgb(0.5, 0.5, 0.5),
        (false, true) => SLIDER_THUMB.lighter(0.3),
        _ => SLIDER_THUMB,
    }
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
        PanOrbitCamera::default(),
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
