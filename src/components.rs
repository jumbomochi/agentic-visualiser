use bevy::prelude::*;

/// Types of agents that can exist in the visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AgentType {
    #[default]
    Main,
    Explore,
    Plan,
    Bash,
    CodeReviewer,
    UIUXReviewer,
    StatuslineSetup,
    ClaudeCodeGuide,
    Haiku,
    DevopsEngineer,
    SecurityAnalyst,
    ProjectManager,
    General,
}

impl AgentType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "explore" => AgentType::Explore,
            "plan" => AgentType::Plan,
            "bash" => AgentType::Bash,
            "code-reviewer" => AgentType::CodeReviewer,
            "ui-ux-reviewer" => AgentType::UIUXReviewer,
            "statusline-setup" => AgentType::StatuslineSetup,
            "claude-code-guide" => AgentType::ClaudeCodeGuide,
            "haiku" => AgentType::Haiku,
            "devops-engineer" => AgentType::DevopsEngineer,
            "security-analyst" => AgentType::SecurityAnalyst,
            "project-manager" => AgentType::ProjectManager,
            "general-purpose" => AgentType::General,
            _ => AgentType::General,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            AgentType::Main => Color::srgb(0.2, 0.6, 1.0),      // Blue
            AgentType::Explore => Color::srgb(0.2, 0.8, 0.4),   // Green
            AgentType::Plan => Color::srgb(0.7, 0.3, 0.9),      // Purple
            AgentType::Bash => Color::srgb(1.0, 0.6, 0.2),      // Orange
            AgentType::CodeReviewer => Color::srgb(1.0, 0.3, 0.3), // Red
            AgentType::UIUXReviewer => Color::srgb(1.0, 0.8, 0.2), // Yellow
            AgentType::StatuslineSetup => Color::srgb(0.8, 0.2, 0.2), // Dark red (Edward's coat)
            AgentType::ClaudeCodeGuide => Color::srgb(0.4, 0.8, 0.4), // Green (Deku)
            AgentType::Haiku => Color::srgb(0.6, 0.6, 0.6),     // Gray (Totoro)
            AgentType::DevopsEngineer => Color::srgb(0.3, 0.8, 0.9), // Cyan (Bulma)
            AgentType::SecurityAnalyst => Color::srgb(0.2, 0.3, 0.6), // Dark blue (Conan)
            AgentType::ProjectManager => Color::srgb(0.4, 0.5, 0.3), // Military green (Erwin)
            AgentType::General => Color::srgb(0.9, 0.8, 0.3),   // Yellow (Pikachu)
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AgentType::Main => "Main",
            AgentType::Explore => "Explore",
            AgentType::Plan => "Plan",
            AgentType::Bash => "Bash",
            AgentType::CodeReviewer => "Review",
            AgentType::UIUXReviewer => "UI/UX",
            AgentType::StatuslineSetup => "Setup",
            AgentType::ClaudeCodeGuide => "Guide",
            AgentType::Haiku => "Haiku",
            AgentType::DevopsEngineer => "DevOps",
            AgentType::SecurityAnalyst => "Security",
            AgentType::ProjectManager => "PM",
            AgentType::General => "Agent",
        }
    }
}

/// Agent component - identifies an entity as an agent
#[derive(Component)]
pub struct Agent {
    pub id: String,
    pub agent_type: AgentType,
    pub tool_use_id: Option<String>,
}

/// Types of workstations in the workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StationType {
    Library,    // Read, Grep, Glob
    Desk,       // Write, Edit
    Terminal,   // Bash
    WebPortal,  // WebFetch, WebSearch
    MeetingArea, // Task (spawn point)
    Center,     // Idle position
}

impl StationType {
    pub fn for_tool(tool_name: &str) -> Self {
        match tool_name {
            "Read" | "Grep" | "Glob" => StationType::Library,
            "Write" | "Edit" => StationType::Desk,
            "Bash" => StationType::Terminal,
            "WebFetch" | "WebSearch" => StationType::WebPortal,
            "Task" => StationType::MeetingArea,
            _ => StationType::Center,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            StationType::Library => Color::srgb(0.6, 0.4, 0.2),    // Brown
            StationType::Desk => Color::srgb(0.8, 0.7, 0.5),       // Tan
            StationType::Terminal => Color::srgb(0.2, 0.2, 0.2),   // Dark gray
            StationType::WebPortal => Color::srgb(0.3, 0.7, 0.9),  // Cyan
            StationType::MeetingArea => Color::srgb(0.5, 0.5, 0.6), // Blue-gray
            StationType::Center => Color::srgb(0.4, 0.4, 0.4),     // Gray
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            StationType::Library => "Library",
            StationType::Desk => "Desk",
            StationType::Terminal => "Terminal",
            StationType::WebPortal => "Web",
            StationType::MeetingArea => "Meeting",
            StationType::Center => "Center",
        }
    }
}

/// Station component - marks a workstation entity
#[derive(Component)]
pub struct Station {
    pub station_type: StationType,
}

/// Movement target for an agent
#[derive(Component)]
pub struct MovementTarget {
    pub position: Vec2,
    pub station_type: Option<StationType>,
}

/// Movement speed
#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(150.0) // pixels per second
    }
}

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    WalkingRight,
    WalkingLeft,
    WalkingUp,
    WalkingDown,
    Working,
}

/// Animation controller component
#[derive(Component)]
pub struct AnimationController {
    pub state: AnimationState,
    pub frame: usize,
    pub timer: Timer,
}

impl Default for AnimationController {
    fn default() -> Self {
        AnimationController {
            state: AnimationState::Idle,
            frame: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

/// Marks an agent as currently working at a station
#[derive(Component)]
pub struct Working {
    pub tool_name: String,
    pub started_at: f32,
}

/// Agent name label entity
#[derive(Component)]
pub struct AgentLabel;

/// Status bar UI component
#[derive(Component)]
pub struct StatusBar;

/// Current tool display
#[derive(Component)]
pub struct CurrentToolText;

/// Session info display
#[derive(Component)]
pub struct SessionText;

/// Agent count display
#[derive(Component)]
pub struct AgentCountText;
