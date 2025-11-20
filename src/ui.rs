use crate::{
    BirdState, RebuildBird,
    bird::{BirdGenInputTypes, BirdGenInputs, get_input_type_string, get_input_value_for_type},
};
use accesskit::{Node as Accessible, Role};
use bevy::{
    a11y::AccessibilityNode,
    color::palettes::basic::*,
    input::mouse::{MouseScrollUnit, MouseWheel},
    input_focus::tab_navigation::TabGroup,
    picking::hover::{HoverMap, Hovered},
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{
        Activate, Button, CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue,
        TrackClick, UiWidgetsPlugins, ValueChange, observe,
    },
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub struct BirdUIPlugin;
impl Plugin for BirdUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiWidgetsPlugins)
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    send_scroll_events,
                    update_slider_values,
                    update_slider_styles,
                    update_slider_styles2,
                    update_button_style,
                    update_button_style2,
                ),
            )
            .add_observer(on_scroll_handler);
    }
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
struct RegenerateButton;

#[derive(Component)]
struct BirdInputSlider {
    input_type: BirdGenInputTypes,
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    /// Scroll delta in logical coordinates.
    delta: Vec2,
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

const LINE_HEIGHT: f32 = 40.0;

fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ui_root(&asset_server));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(24),
            left: px(24),
            ..default()
        },
        children![
            Text::new("bird-o-matic"),
            TextFont {
                font: asset_server.load("fonts/OTBrut-Regular.ttf"),
                font_size: 48.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ],
    ));
}

fn ui_root(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            bottom: px(4),
            left: px(24),
            right: px(24),
            min_width: vw(33),
            max_width: vw(75),
            max_height: vh(30),
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(px(30.), px(20)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
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
            font_size: 24.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Node {
            margin: UiRect::top(px(10)).with_bottom(px(4)),
            ..default()
        },
    )
}

fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: px(1.),
            margin: UiRect::vertical(px(16.)),
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
            margin: UiRect::vertical(px(5.)),
            min_height: px(LINE_HEIGHT),
            max_height: px(LINE_HEIGHT),
            ..default()
        },
        children![
            AccessibilityNode(Accessible::new(Role::ListItem)),
            // Label
            (
                Text::new(get_input_type_string(&input_type)),
                TextFont {
                    font: asset_server.load("fonts/OTBrut-Regular.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(px(3.)),
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
                        height: px(12.),
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
                                height: px(6.),
                                ..default()
                            },
                            BackgroundColor(SLIDER_TRACK),
                            BorderRadius::all(px(3.)),
                        ),
                        // Invisible track for thumb positioning
                        (
                            Node {
                                display: Display::Flex,
                                position_type: PositionType::Absolute,
                                left: px(0.),
                                right: px(12.),
                                top: px(0.),
                                bottom: px(0.),
                                ..default()
                            },
                            children![
                                // Thumb
                                (
                                    SliderThumb,
                                    Node {
                                        display: Display::Flex,
                                        width: px(12.),
                                        height: px(12.),
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
            min_height: px(40.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(px(5.)),
            padding: UiRect::axes(px(8.), px(16)),
            border: UiRect::all(px(2.)),
            ..default()
        },
        Button,
        RegenerateButton,
        Hovered::default(),
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(Color::BLACK),
        BorderRadius::all(px(5.)),
        children![(
            Text::new("Regenerate Bird"),
            TextFont {
                font: asset_server.load("fonts/OTBrut-Regular.ttf"),
                font_size: 20.0,
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
            margin: UiRect::top(px(10.)),
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
    mut sliders: Query<(Entity, &BirdInputSlider)>,
    mut commands: Commands,
) {
    if res.is_changed() {
        for (slider_ent, bird_slider) in sliders.iter_mut() {
            commands
                .entity(slider_ent)
                .insert(SliderValue(get_input_value_for_type(
                    &bird_slider.input_type,
                    &res,
                )));
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
