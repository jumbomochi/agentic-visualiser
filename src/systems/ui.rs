use bevy::prelude::*;

use crate::components::*;
use crate::resources::GameState;

/// System to set up the UI
pub fn setup_ui(mut commands: Commands) {
    // Status bar at the bottom
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            StatusBar,
        ))
        .with_children(|parent| {
            // Current tool text
            parent.spawn((
                Text::new("[Idle]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                CurrentToolText,
            ));

            // Session info
            parent.spawn((
                Text::new("Session: -"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                SessionText,
            ));

            // Agent count
            parent.spawn((
                Text::new("Agents: 1"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                AgentCountText,
            ));
        });

    // Title bar at the top
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            padding: UiRect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.2, 0.9)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Claude Code Visualiser"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.4, 0.7, 1.0)),
        ));
    });
}

/// System to update UI text
pub fn update_ui_system(
    game_state: Res<GameState>,
    mut tool_text: Query<&mut Text, (With<CurrentToolText>, Without<SessionText>, Without<AgentCountText>)>,
    mut session_text: Query<&mut Text, (With<SessionText>, Without<CurrentToolText>, Without<AgentCountText>)>,
    mut agent_text: Query<&mut Text, (With<AgentCountText>, Without<CurrentToolText>, Without<SessionText>)>,
) {
    // Update tool text
    if let Ok(mut text) = tool_text.get_single_mut() {
        if let (Some(tool), Some(input)) = (&game_state.current_tool, &game_state.current_tool_input) {
            **text = format!("[{}] {}", tool, input);
        } else {
            **text = "[Idle]".to_string();
        }
    }

    // Update session text
    if let Ok(mut text) = session_text.get_single_mut() {
        if let Some(session) = &game_state.session_id {
            let short = if session.len() > 8 {
                &session[..8]
            } else {
                session
            };
            **text = format!("Session: {}", short);
        }
    }

    // Update agent count
    if let Ok(mut text) = agent_text.get_single_mut() {
        **text = format!("Agents: {}", game_state.agent_count);
    }
}

/// System plugin for UI
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui_system);
    }
}
