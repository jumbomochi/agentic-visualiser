use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

use crate::events::parse_event;
use crate::resources::{EventQueue, FileWatcherState, GameState};

/// System that reads new events from the JSONL file
pub fn read_events_system(
    mut file_state: ResMut<FileWatcherState>,
    mut event_queue: ResMut<EventQueue>,
    mut game_state: ResMut<GameState>,
) {
    // Try to open and read the events file
    let Ok(file) = File::open(&file_state.events_path) else {
        return;
    };

    let mut reader = BufReader::new(file);

    // Seek to last known position
    if reader.seek(SeekFrom::Start(file_state.last_position)).is_err() {
        return;
    }

    // Read new lines
    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim();
        if !trimmed.is_empty() {
            if let Some(event) = parse_event(trimmed) {
                // Detect session change - mark for agent cleanup
                let session_changed = game_state.session_id.as_ref()
                    != Some(&event.session_id);

                if session_changed {
                    game_state.session_changed = true;
                }

                // Update game state
                game_state.session_id = Some(event.session_id.clone());
                game_state.current_tool = Some(event.tool_name.clone());
                game_state.current_tool_input = Some(event.input_summary());
                game_state.events_processed += 1;

                // Queue the event for processing
                event_queue.push(event);
            }
        }
        line.clear();
    }

    // Update position
    if let Ok(pos) = reader.stream_position() {
        file_state.last_position = pos;
    }
}

/// System plugin for event reading
pub struct EventReaderPlugin;

impl Plugin for EventReaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FileWatcherState>()
            .init_resource::<EventQueue>()
            .init_resource::<GameState>()
            .add_systems(Update, read_events_system);
    }
}
