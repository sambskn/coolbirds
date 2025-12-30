use bevy::prelude::*;

use crate::ui::{FONT_PATH_OT_BRUT_REGULAR, TEXT_COLOR};

const LOG_LIFETIME_S: f32 = 5.;

pub struct LogTextPlugin;
impl Plugin for LogTextPlugin {
    fn build(&self, app: &mut App) {
        // Register message type
        app.add_message::<NewLog>()
            .insert_resource(LogTracker { logs: vec![] })
            .add_systems(Update, (handle_new_log, update_existing_logs));
    }
}

fn handle_new_log(
    mut new_log_reader: MessageReader<NewLog>,
    mut log_tracker: ResMut<LogTracker>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for log_message in new_log_reader.read() {
        let tracker = log_tracker.as_mut();
        let log_id = tracker.logs.len() + 1;
        tracker.logs.push(LogMessage {
            created_at: time.elapsed_secs(),
            id: log_id,
        });
        commands.spawn((
            LogText(log_id),
            Node {
                position_type: PositionType::Absolute,
                top: px(16 + (log_id as i32 - 1) * 36),
                left: px(16),
                ..default()
            },
            Text::new(log_message.text.clone()),
            TextFont {
                font: asset_server.load(FONT_PATH_OT_BRUT_REGULAR),
                font_size: 16.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
    }
}

fn update_existing_logs(
    text_query: Query<(&LogText, Entity)>,
    mut log_tracker: ResMut<LogTracker>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let tracker = log_tracker.as_mut();
    for (log_text, entity) in text_query {
        let message = tracker.logs.iter().find(|&log| log.id == log_text.0);
        match message {
            Some(log) => {
                if log.created_at + LOG_LIFETIME_S < time.elapsed_secs() {
                    // Kill the log
                    let mut ent = commands.get_entity(entity).unwrap();
                    ent.despawn();
                    // Remove from tracker
                    tracker.logs = tracker
                        .logs
                        .iter()
                        .filter(|&log| log.id != log_text.0)
                        .map(|log| *log)
                        .collect();
                }
            }
            None => {
                info!("No matching log found??");
                // Kill the log anyway
                let mut ent = commands.get_entity(entity).unwrap();
                ent.despawn();
            }
        }
    }
}

#[derive(Message, Debug)]
pub struct NewLog {
    pub text: String,
}

#[derive(Component)]
struct LogText(usize);

#[derive(Clone, Copy)]
struct LogMessage {
    pub created_at: f32,
    pub id: usize,
}

#[derive(Resource)]
struct LogTracker {
    pub logs: Vec<LogMessage>,
}
