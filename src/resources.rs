use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::components::StationType;
use crate::events::ToolEvent;

/// Shared game state resource
#[derive(Resource, Default)]
pub struct GameState {
    pub session_id: Option<String>,
    pub current_tool: Option<String>,
    pub current_tool_input: Option<String>,
    pub agent_count: usize,
    pub events_processed: usize,
    /// Flag indicating session changed - agents should be cleared
    pub session_changed: bool,
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

        // Start from end of file to skip historical events
        let last_position = std::fs::metadata(&events_path)
            .map(|m| m.len())
            .unwrap_or(0);

        FileWatcherState {
            events_path,
            last_position,
        }
    }
}

/// Tracks agents at each station for orbital positioning
#[derive(Resource, Default)]
pub struct StationOccupancy {
    /// Maps station type to list of agent entity IDs currently there
    pub agents_at_station: HashMap<StationType, Vec<Entity>>,
}

impl StationOccupancy {
    /// Calculate orbital position for an agent at a station
    /// Returns an offset from the station center
    pub fn get_orbital_offset(&self, station: StationType, agent_entity: Entity) -> Vec2 {
        let agents = self.agents_at_station.get(&station).map(|v| v.as_slice()).unwrap_or(&[]);

        if agents.len() <= 1 {
            return Vec2::ZERO;
        }

        // Find this agent's index in the station
        let index = agents.iter().position(|&e| e == agent_entity).unwrap_or(0);

        // Calculate orbital position
        let orbit_radius = 45.0; // Distance from center
        let total_agents = agents.len();
        let angle = (index as f32 / total_agents as f32) * std::f32::consts::TAU;

        Vec2::new(
            angle.cos() * orbit_radius,
            angle.sin() * orbit_radius,
        )
    }

    /// Add an agent to a station
    pub fn add_agent(&mut self, station: StationType, entity: Entity) {
        // First remove from any other station
        self.remove_agent(entity);
        // Then add to new station
        self.agents_at_station.entry(station).or_default().push(entity);
    }

    /// Remove an agent from all stations
    pub fn remove_agent(&mut self, entity: Entity) {
        for agents in self.agents_at_station.values_mut() {
            agents.retain(|&e| e != entity);
        }
    }

    /// Get the count of agents at a station
    pub fn count_at_station(&self, station: StationType) -> usize {
        self.agents_at_station.get(&station).map(|v| v.len()).unwrap_or(0)
    }
}
