use bevy::prelude::*;
use std::collections::VecDeque;

use crate::events::ToolEvent;

/// Shared game state resource
#[derive(Resource, Default)]
pub struct GameState {
    pub session_id: Option<String>,
    pub current_tool: Option<String>,
    pub current_tool_input: Option<String>,
    pub agent_count: usize,
    pub events_processed: usize,
}

/// Queue of tool events to process
#[derive(Resource, Default)]
pub struct EventQueue {
    pub events: VecDeque<ToolEvent>,
}

impl EventQueue {
    pub fn push(&mut self, event: ToolEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<ToolEvent> {
        self.events.pop_front()
    }
}

/// Station positions in the workspace
#[derive(Resource)]
pub struct StationPositions {
    pub library: Vec2,
    pub desk: Vec2,
    pub terminal: Vec2,
    pub web_portal: Vec2,
    pub meeting_area: Vec2,
    pub center: Vec2,
}

impl Default for StationPositions {
    fn default() -> Self {
        // Positions relative to center of screen (800x600)
        StationPositions {
            library: Vec2::new(-250.0, 100.0),      // Top left
            terminal: Vec2::new(250.0, 100.0),      // Top right
            desk: Vec2::new(-250.0, -100.0),        // Bottom left
            web_portal: Vec2::new(250.0, -100.0),   // Bottom right
            meeting_area: Vec2::new(0.0, 150.0),    // Top center
            center: Vec2::new(0.0, 0.0),            // Center
        }
    }
}

impl StationPositions {
    pub fn get(&self, station_type: crate::components::StationType) -> Vec2 {
        use crate::components::StationType;
        match station_type {
            StationType::Library => self.library,
            StationType::Desk => self.desk,
            StationType::Terminal => self.terminal,
            StationType::WebPortal => self.web_portal,
            StationType::MeetingArea => self.meeting_area,
            StationType::Center => self.center,
        }
    }
}

/// File watcher state
#[derive(Resource)]
pub struct FileWatcherState {
    pub events_path: std::path::PathBuf,
    pub last_position: u64,
}

impl Default for FileWatcherState {
    fn default() -> Self {
        let events_path = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".claude-visualizer")
            .join("events.jsonl");

        FileWatcherState {
            events_path,
            last_position: 0,
        }
    }
}
