use crate::{
    BG_COLOR, BirdSTLContents, BirdState, RebuildBird,
    bird::{BirdGenInputs, RecentBirds},
    log_text::NewLog,
    random_words::get_bird_description,
};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{Activate, Button, UiWidgetsPlugins, observe},
};
use bevy_file_dialog::FileDialogExt;
use bevy_mod_clipboard::{Clipboard, ClipboardRead};

const NORMAL_BUTTON: Color = Color::srgba(0., 0., 0., 0.00);
const HOVERED_BUTTON: Color = Color::srgba(1.0, 1.0, 1.0, 0.95);
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const FADED_TEXT_COLOR: Color = Color::srgba(0.9, 0.9, 0.9, 0.9);
const HOVERED_TEXT_COLOR: Color = BG_COLOR;
const DISABLED_TEXT_COLOR: Color = Color::srgba(0.7, 0.8, 0.85, 0.7);

pub const FONT_PATH_OT_BRUT_REGULAR: &str = "fonts/OTBrut-Regular.ttf";
pub const FONT_PATH_MONTREAL: &str = "fonts/OTNeueMontreal-BoldItalicSqueezed.ttf";

const BIRD_CHOICE_LABEL_FONT_SIZE: f32 = 48.;
const BIRD_CHOICE_DESCRIPTION_FONT_SIZE: f32 = 14.;

const BIRD_CHOICE_VW_EDGE_PUSH: i32 = 10;

pub struct BirdUIPlugin;
impl Plugin for BirdUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiWidgetsPlugins)
            .insert_resource::<PasteWatcher>(PasteWatcher(None))
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_button_style,
                    update_button_style2,
                    update_button_text_style,
                    listen_for_pasted_values,
                    handle_bird_rebuild,
                ),
            );
    }
}

#[derive(Component)]
struct BirdDescription;

fn handle_bird_rebuild(
    mut bird_rebuild_reader: MessageReader<RebuildBird>,
    mut bird_descriptions: Query<&mut Text, With<BirdDescription>>,
) {
    for _event in bird_rebuild_reader.read() {
        for mut bird_description in &mut bird_descriptions {
            bird_description.0 = get_bird_description();
        }
    }
}

#[derive(Resource)]
struct PasteWatcher(Option<ClipboardRead>);

fn listen_for_pasted_values(
    mut paste_watcher: ResMut<PasteWatcher>,
    mut bird_inputs: ResMut<BirdGenInputs>,
    mut rebuild_writer: MessageWriter<RebuildBird>,
    mut log_writer: MessageWriter<NewLog>,
) {
    if let Some(read) = &mut paste_watcher.0 {
        if let Some(contents) = read.poll_result() {
            let clipboard_contents = contents.unwrap_or_else(|e| format!("{e:?}"));
            // Now actually update da bird??
            match bird_inputs.update_from_seed_string(clipboard_contents.clone()) {
                Ok(()) => {
                    log_writer.write(NewLog {
                        text: format!(
                            "loaded bird seed from clipboard\n{}",
                            clipboard_contents.clone()
                        ),
                    });
                    info!("Bird updated successfully!");
                    rebuild_writer.write(RebuildBird);
                }
                Err(e) => {
                    info!("Error parsing seed: {}", e);
                    log_writer.write(NewLog {
                        text: "oof that didn't work".to_string(),
                    });
                }
            }
            paste_watcher.0 = None;
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
            right: px(18),
            bottom: px(12),
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
                    margin: UiRect::vertical(px(-12)),
                    ..default()
                },
                children![
                    (
                        Text::new("coolbirds.website"),
                        TextFont {
                            font: asset_server.load(FONT_PATH_MONTREAL),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),
                    (
                        Text::new("v0.2.2"),
                        TextFont {
                            font: asset_server.load(FONT_PATH_MONTREAL),
                            font_size: 16.0,
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
            top: px(8),
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
                     bird_state: Res<State<BirdState>>,
                     mut log_writer: MessageWriter<NewLog>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            let bird_str = bird_inputs.get_bird_seed_string();
                            clipboard
                                .set_text(bird_str.clone())
                                .expect("Failed to get bird string");
                            log_writer.write(NewLog {
                                text: format!("copied bird seed to clipboard\n{}", bird_str),
                            });
                        }
                    }
                )
            ),
            (
                bird_action_button(&asset_server, "paste".to_string()),
                observe(
                    |_activate: On<Activate>,
                     mut paste_watcher: ResMut<PasteWatcher>,
                     mut clipboard: ResMut<Clipboard>,
                     bird_state: Res<State<BirdState>>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            // If no clipboard read is pending, fetch any text
                            if paste_watcher.0.is_none() {
                                // `fetch_text` completes instantly on windows and unix.
                                // On wasm32 the result is fetched asynchronously, and the `ClipboardRead` needs to stored and polled
                                // on following frames until a result is available.
                                // Our `listen_for_pasted_values` system will pick up the val
                                paste_watcher.0 = Some(clipboard.fetch_text());
                            }
                        }
                    }
                )
            ),
            (
                bird_action_button(&asset_server, "save stl".to_string()),
                observe(
                    |_activate: On<Activate>,
                     bird_inputs: Res<BirdGenInputs>,
                     mut commands: Commands,
                     bird_state: Res<State<BirdState>>,
                     mut log_writer: MessageWriter<NewLog>| {
                        if *bird_state.get() == BirdState::BirdVisible {
                            log_writer.write(NewLog {
                                text: "creating bird STL...".to_string(),
                            });
                            // make bird stl
                            let stl_bytes_result = bird_inputs.get_stl();
                            match stl_bytes_result {
                                Ok(stl_binary) => {
                                    // pop a file dialog for them to save the file
                                    commands
                                        .dialog()
                                        .add_filter("STL", &["stl"])
                                        .set_file_name("coolbird.stl")
                                        .save_file::<BirdSTLContents>(stl_binary);
                                }
                                _ => {
                                    log_writer.write(NewLog {
                                        text: "yikes couldn't make an STL oops".to_string(),
                                    });
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
    let bird_desc_left = get_bird_description();
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: vw(BIRD_CHOICE_VW_EDGE_PUSH),
            height: vh(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Node {
                    margin: UiRect {
                        left: px(0),
                        right: px(40),
                        top: px(0),
                        bottom: px(0)
                    },
                    ..default()
                },
                BirdDescription,
                Text::new(bird_desc_left),
                TextFont {
                    font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                    font_size: BIRD_CHOICE_DESCRIPTION_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ),
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
                Text::new("LEFT"),
                TextFont {
                    font: asset_server.load(FONT_PATH_MONTREAL),
                    font_size: BIRD_CHOICE_LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ),
        ],
    ));

    // right bird label
    let bird_desc_right = get_bird_description();
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: vw(BIRD_CHOICE_VW_EDGE_PUSH),
            height: vh(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Node {
                    margin: UiRect {
                        left: px(0),
                        right: px(4),
                        top: px(0),
                        bottom: px(0)
                    },
                    ..default()
                },
                Text::new("RIGHT"),
                TextFont {
                    font: asset_server.load(FONT_PATH_MONTREAL),
                    font_size: BIRD_CHOICE_LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ),
            (
                Node {
                    margin: UiRect {
                        left: px(60),
                        right: px(0),
                        top: px(0),
                        bottom: px(60)
                    },
                    ..default()
                },
                BirdDescription,
                Text::new(bird_desc_right),
                TextFont {
                    font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                    font_size: BIRD_CHOICE_DESCRIPTION_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ),
        ],
    ));

    // general actions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: vh(11),
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
                    margin: UiRect {
                        left: px(0),
                        right: px(0),
                        top: px(0),
                        bottom: px(0)
                    },
                    ..default()
                },
                Text::new("selected birds become the new seed"),
                TextFont {
                    font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                    font_size: 16.0,
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

    // directions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: vw(5),
            bottom: vh(2),
            max_width: vw(33),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        },
        children![
            (
            Node {
                margin: UiRect {
                    left: px(0),
                    right: px(0),
                    top: px(0),
                    bottom: px(-4)
                },
                ..default()
            },
            Text::new("INSTRUCTIONS"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 28.0,
                ..default()
            },
            TextColor(FADED_TEXT_COLOR),
        ),
        (
            Node {
                margin: UiRect {
                    left: px(-30),
                    right: px(0),
                    top: px(0),
                    bottom: px(0)
                },
                ..default()
            },
            Text::new("~ rotate birds with dragging motions\n   ~ zooming can also be done\n\t~ select the best birds"),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 12.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )
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
        children![(
            Hovered::default(),
            Text::new(text),
            TextFont {
                font: asset_server.load(FONT_PATH_MONTREAL),
                font_size: 26.0,
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
        children![(
            Hovered::default(),
            Text::new(text),
            TextFont {
                font: asset_server.load(FONT_PATH_MONTREAL),
                font_size: 26.0,
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
        (&Hovered, Has<InteractionDisabled>, &mut BackgroundColor),
        (
            Or<(Changed<Hovered>, Added<InteractionDisabled>)>,
            Or<(With<Button>,)>,
        ),
    >,
) {
    for (hovered, disabled, mut color) in &mut buttons {
        set_button_style(disabled, hovered.get(), &mut color);
    }
}

fn update_button_style2(
    mut buttons: Query<
        (&Hovered, Has<InteractionDisabled>, &mut BackgroundColor),
        Or<(With<Button>,)>,
    >,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_disabled.read().for_each(|entity| {
        if let Ok((hovered, disabled, mut color)) = buttons.get_mut(entity) {
            set_button_style(disabled, hovered.get(), &mut color);
        }
    });
}

fn update_button_text_style(
    mut texts: Query<(&Hovered, Has<InteractionDisabled>, &mut TextColor)>,
) {
    for (hovered, disabled, mut text_color) in &mut texts {
        match (disabled, hovered.0) {
            (true, _) => {
                text_color.0 = DISABLED_TEXT_COLOR;
            }
            (false, true) => {
                text_color.0 = HOVERED_TEXT_COLOR;
            }
            (false, false) => {
                text_color.0 = TEXT_COLOR;
            }
        }
    }
}

fn set_button_style(disabled: bool, hovered: bool, color: &mut BackgroundColor) {
    match (disabled, hovered) {
        (true, _) => {
            *color = NORMAL_BUTTON.into();
        }
        (false, true) => {
            *color = HOVERED_BUTTON.into();
        }
        (false, false) => {
            *color = NORMAL_BUTTON.into();
        }
    }
}
