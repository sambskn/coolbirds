use crate::{
    BirdState, RebuildBird,
    bird::{BirdGenInputs, RecentBirds},
};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::{HoverMap, Hovered},
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{Activate, Button, UiWidgetsPlugins, observe},
};
use bevy_mod_clipboard::{Clipboard, ClipboardRead};

const NORMAL_BUTTON: Color = Color::srgba(0.15, 0.15, 0.15, 0.01);
const HOVERED_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.1);
const NORMAL_BUTTON_BORDER: Color = Color::Srgba(Srgba {
    red: 0.02,
    green: 0.12,
    blue: 0.33,
    alpha: 0.1,
});
const HOVERED_BUTTON_BORDER: Color = Color::Srgba(Srgba {
    red: 0.9,
    green: 0.92,
    blue: 0.9,
    alpha: 0.5,
});
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const FONT_PATH_OT_BRUT_REGULAR: &str = "fonts/OTBrut-Regular.ttf";
const FONT_PATH_ACMA_BOLD: &str = "fonts/PPAcma-Bold.ttf";

pub struct BirdUIPlugin;
impl Plugin for BirdUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiWidgetsPlugins)
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    send_scroll_events,
                    update_button_style,
                    update_button_style2,
                ),
            )
            .add_observer(on_scroll_handler);
    }
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

const LINE_HEIGHT: f32 = 54.0;

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
    commands.spawn((
        Camera2d,
        Camera {
            order: 3,
            ..default()
        },
        IsDefaultUiCamera,
    ));

    // app label
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: px(24),
            bottom: px(24),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                children![
                    (
                        Text::new("bird-o-matic"),
                        TextFont {
                            font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),
                    (
                        Text::new("v0.2.0"),
                        TextFont {
                            font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    )
                ]
            ),
            footer(&asset_server)
        ],
    ));
    // seed bird labels
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(48),
            width: vw(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        children![(
            Node {
                margin: UiRect {
                    left: px(275),
                    right: px(0),
                    top: px(0),
                    bottom: px(0)
                },
                ..default()
            },
            Text::new("SEED BIRD"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 32.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ), (
            Node {
                margin: UiRect {
                    left: px(326),
                    right: px(0),
                    top: px(-10),
                    bottom: px(0)
                },
                max_width: vw(40),
                ..default()
            },
            Text::new("bird of origin, it's 'genome' will be mixed with algorithm-provided birds to create new progeny"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 12.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    ));

    // seed bird actions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(32),
            right: vw(50),
            max_width: vw(45),
            padding: UiRect {
                left: px(0),
                right: px(100),
                top: px(0),
                bottom: px(0),
            },
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::End,
            ..default()
        },
        children![
            (
                bird_action_button(&asset_server, "copy".to_string()),
                observe(
                    |_activate: On<Activate>,
                     mut clipboard: ResMut<Clipboard>,
                     bird_inputs: Res<BirdGenInputs>,
                     bird_state: Res<State<BirdState>>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            let bird_str = bird_inputs.get_bird_seed_string();
                            clipboard
                                .set_text(bird_str)
                                .expect("Failed to get bird string");
                        }
                    }
                )
            ),
            (
                bird_action_button(&asset_server, "paste".to_string()),
                observe(
                    |_activate: On<Activate>,
                     mut bird_inputs: ResMut<BirdGenInputs>,
                     mut rebuild_writer: MessageWriter<RebuildBird>,
                     mut maybe_read: Local<Option<ClipboardRead>>,
                     mut clipboard: ResMut<Clipboard>,
                     bird_state: Res<State<BirdState>>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            // If no clipboard read is pending, fetch any text
                            if maybe_read.is_none() {
                                // `fetch_text` completes instantly on windows and unix.
                                // On wasm32 the result is fetched asynchronously, and the `ClipboardRead` needs to stored and polled
                                // on following frames until a result is available.
                                *maybe_read = Some(clipboard.fetch_text());
                            }

                            // Check if the clipboard read is complete and, if so, display its result.
                            if let Some(read) = maybe_read.as_mut() {
                                if let Some(contents) = read.poll_result() {
                                    let clipboard_contents =
                                        contents.unwrap_or_else(|e| format!("{e:?}"));
                                    // Now actually update da bird??
                                    match bird_inputs.update_from_seed_string(clipboard_contents) {
                                        Ok(()) => {
                                            info!("Bird updated successfully!");
                                            rebuild_writer.write(RebuildBird);
                                        }
                                        Err(e) => info!("Error parsing seed: {}", e),
                                    }
                                    *maybe_read = None;
                                }
                            }
                        }
                    }
                )
            ),
            (
                bird_action_button(&asset_server, "randomize".to_string()),
                observe(
                    |_activate: On<Activate>,
                     mut bird_inputs: ResMut<BirdGenInputs>,
                     mut rebuild_writer: MessageWriter<RebuildBird>,
                     bird_state: Res<State<BirdState>>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            bird_inputs.randomize_values();
                            rebuild_writer.write(RebuildBird);
                        }
                    }
                )
            ),
        ],
    ));

    // left bird label
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: vw(18),
            height: vh(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                margin: UiRect {
                    left: px(0),
                    right: px(0),
                    top: px(20),
                    bottom: px(0)
                },
                ..default()
            },
            Text::new("LEFT"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 32.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    ));

    // right bird label
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: vw(18),
            height: vh(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                margin: UiRect {
                    left: px(0),
                    right: px(0),
                    top: px(0),
                    bottom: px(60)
                },
                ..default()
            },
            Text::new("RIGHT"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 32.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    ));

    // general actions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: vh(5),
            height: vh(40),
            left: vw(35),
            right: vw(35),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            ..default()
        },
        children![
            (
                Node {
                    margin: UiRect {
                        left: px(0),
                        right: px(0),
                        top: px(0),
                        bottom: px(0)
                    },
                    ..default()
                },
                Text::new("SELECT BIRD"),
                TextFont {
                    font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::horizontal(px(12)),
                    ..default()
                },
                children![
                    (
                        bird_selection_button(&asset_server, "left bird".to_string()),
                        observe(
                            |_activate: On<Activate>,
                             mut bird_inputs: ResMut<BirdGenInputs>,
                             recent_birds: Res<RecentBirds>,
                             mut rebuild_writer: MessageWriter<RebuildBird>,
                             bird_state: Res<State<BirdState>>| {
                                if *bird_state.get() == BirdState::BirdVisible {
                                    // set bird inputs to be equal to left bird
                                    bird_inputs.copy_from_other_bird(&recent_birds.left);
                                    rebuild_writer.write(RebuildBird);
                                }
                            }
                        ),
                    ),
                    (
                        bird_selection_button(&asset_server, "right bird".to_string()),
                        observe(
                            |_activate: On<Activate>,
                             mut bird_inputs: ResMut<BirdGenInputs>,
                             recent_birds: Res<RecentBirds>,
                             mut rebuild_writer: MessageWriter<RebuildBird>,
                             bird_state: Res<State<BirdState>>| {
                                if *bird_state.get() == BirdState::BirdVisible {
                                    // set bird inputs to be equal to right bird
                                    bird_inputs.copy_from_other_bird(&recent_birds.right);
                                    rebuild_writer.write(RebuildBird);
                                }
                            }
                        ),
                    ),
                ]
            ),
        ],
    ));
}

fn bird_action_button(asset_server: &AssetServer, text: String) -> impl Bundle {
    (
        Node {
            max_width: vw(10),
            min_height: px(40.),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(px(2)),
            padding: UiRect::axes(px(8.), px(0.)),
            border: UiRect::bottom(px(4)),
            ..default()
        },
        Button,
        Hovered::default(),
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(NORMAL_BUTTON_BORDER),
        BorderRadius::all(px(2.)),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load(FONT_PATH_ACMA_BOLD),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}

fn bird_selection_button(asset_server: &AssetServer, text: String) -> impl Bundle {
    (
        Node {
            max_width: vw(30),
            min_height: px(60.),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            margin: UiRect::horizontal(px(4)),
            padding: UiRect::axes(px(12.), px(16.)),
            border: UiRect::bottom(px(4)),
            ..default()
        },
        Button,
        Hovered::default(),
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(NORMAL_BUTTON_BORDER),
        BorderRadius::all(px(2.)),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load(FONT_PATH_ACMA_BOLD),
                font_size: 24.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}

fn footer(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new("inspired by the OpenSCAD script 'bird-o-matic' by mooncactus"),
        TextFont {
            font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.85, 0.9)),
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
            Or<(With<Button>,)>,
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
        Or<(With<Button>,)>,
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
            border_color.set_all(NORMAL_BUTTON_BORDER);
        }
        (false, true) => {
            *color = HOVERED_BUTTON.into();
            border_color.set_all(HOVERED_BUTTON_BORDER);
        }
        (false, false) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(NORMAL_BUTTON_BORDER);
        }
    }
}
