use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::events::EventType;
use crate::resources::{EventQueue, GameState, StationPositions};
use crate::sprites::SpriteAssets;

/// Marker for the main agent
#[derive(Component)]
pub struct MainAgent;

/// System to spawn the main agent at startup
pub fn spawn_main_agent(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    station_positions: Res<StationPositions>,
    sprite_assets: Res<SpriteAssets>,
) {
    let center = station_positions.center;

    // Get sprite for main agent
    if let Some(image_handle) = sprite_assets.agents.get(&AgentType::Main) {
        commands
            .spawn((
                Sprite {
                    image: image_handle.clone(),
                    custom_size: Some(Vec2::new(48.0, 48.0)), // Scale up 1.5x
                    ..default()
                },
                Transform::from_xyz(center.x, center.y, 10.0),
                Agent {
                    id: "main".to_string(),
                    agent_type: AgentType::Main,
                    tool_use_id: None,
                },
                MainAgent,
                Speed::default(),
                AnimationController::default(),
            ))
            .with_children(|parent| {
                // Name label above character
                parent.spawn((
                    Sprite {
                        color: Color::srgba(0.2, 0.5, 0.9, 0.8),
                        custom_size: Some(Vec2::new(40.0, 16.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 32.0, 0.5),
                ));
                parent.spawn((
                    Text2d::new("Main"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 32.0, 1.0),
                ));
            });
    }

    game_state.agent_count = 1;
}

/// System to process events and spawn/move agents
pub fn process_events_system(
    mut commands: Commands,
    mut event_queue: ResMut<EventQueue>,
    mut game_state: ResMut<GameState>,
    station_positions: Res<StationPositions>,
    sprite_assets: Res<SpriteAssets>,
    mut agents: Query<(Entity, &Agent, &mut Transform), Without<MainAgent>>,
    mut main_agent: Query<(Entity, &mut Transform), With<MainAgent>>,
) {
    while let Some(event) = event_queue.pop() {
        match event.event_type {
            EventType::PreToolUse => {
                let station_type = StationType::for_tool(&event.tool_name);
                let target_pos = station_positions.get(station_type);

                // If it's a Task tool, spawn a new subagent
                if event.tool_name == "Task" {
                    if let Some(subagent_type_str) = event.get_subagent_type() {
                        let agent_type = AgentType::from_str(&subagent_type_str);

                        // Spawn at meeting area with slight random offset
                        let mut rng = rand::thread_rng();
                        let offset = Vec2::new(
                            rng.gen_range(-30.0..30.0),
                            rng.gen_range(-30.0..30.0),
                        );
                        let spawn_pos = station_positions.meeting_area + offset;

                        // Get sprite for this agent type
                        if let Some(image_handle) = sprite_assets.agents.get(&agent_type) {
                            commands
                                .spawn((
                                    Sprite {
                                        image: image_handle.clone(),
                                        custom_size: Some(Vec2::new(42.0, 42.0)),
                                        ..default()
                                    },
                                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 9.0),
                                    Agent {
                                        id: event.tool_use_id.clone(),
                                        agent_type,
                                        tool_use_id: Some(event.tool_use_id.clone()),
                                    },
                                    Speed(180.0), // Subagents move faster
                                    AnimationController::default(),
                                    MovementTarget {
                                        position: spawn_pos,
                                        station_type: Some(StationType::MeetingArea),
                                    },
                                ))
                                .with_children(|parent| {
                                    // Name label
                                    let label_color = agent_type.color();
                                    parent.spawn((
                                        Sprite {
                                            color: Color::srgba(
                                                label_color.to_srgba().red * 0.7,
                                                label_color.to_srgba().green * 0.7,
                                                label_color.to_srgba().blue * 0.7,
                                                0.9,
                                            ),
                                            custom_size: Some(Vec2::new(50.0, 14.0)),
                                            ..default()
                                        },
                                        Transform::from_xyz(0.0, 28.0, 0.5),
                                    ));
                                    parent.spawn((
                                        Text2d::new(agent_type.label()),
                                        TextFont {
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        Transform::from_xyz(0.0, 28.0, 1.0),
                                    ));
                                });

                            game_state.agent_count += 1;
                        }
                    }
                } else {
                    // Move main agent to the appropriate station
                    if let Ok((entity, _transform)) = main_agent.get_single_mut() {
                        commands.entity(entity).insert(MovementTarget {
                            position: target_pos,
                            station_type: Some(station_type),
                        });
                    }
                }
            }
            EventType::PostToolUse => {
                // Tool completed
                if event.tool_name == "Task" {
                    // Find and despawn the subagent with fade out effect
                    for (entity, agent, _transform) in agents.iter_mut() {
                        if agent.tool_use_id.as_ref() == Some(&event.tool_use_id) {
                            // TODO: Add fade out animation
                            commands.entity(entity).despawn_recursive();
                            game_state.agent_count = game_state.agent_count.saturating_sub(1);
                            break;
                        }
                    }
                }
            }
            EventType::SubagentStop => {
                // Already handled by PostToolUse for Task
            }
            EventType::Unknown => {}
        }
    }
}

/// System plugin for agent management
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationPositions>()
            .add_systems(
                Startup,
                spawn_main_agent.after(crate::sprites::generate_sprites),
            )
            .add_systems(Update, process_events_system);
    }
}
