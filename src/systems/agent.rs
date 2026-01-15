use bevy::prelude::*;

use crate::components::*;
use crate::events::EventType;
use crate::resources::{EventQueue, GameState, StationOccupancy, StationPositions};
use crate::sprites::SpriteAssets;

/// Marker for the main agent
#[derive(Component)]
pub struct MainAgent;

/// System to spawn the main agent at startup
pub fn spawn_main_agent(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut station_occupancy: ResMut<StationOccupancy>,
    station_positions: Res<StationPositions>,
    sprite_assets: Res<SpriteAssets>,
) {
    let center = station_positions.center;

    // Get sprite for main agent
    if let Some(image_handle) = sprite_assets.agents.get(&AgentType::Main) {
        let entity = commands
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
                CurrentStation { station: Some(StationType::Center) },
                LabelStagger { index: 0 },
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
                    AgentLabel,
                ));
                parent.spawn((
                    Text2d::new("Main"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 32.0, 1.0),
                    AgentLabel,
                ));
            })
            .id();

        // Register with station occupancy
        station_occupancy.add_agent(StationType::Center, entity);
    }

    game_state.agent_count = 1;
}

/// System to process events and spawn/move agents
pub fn process_events_system(
    mut commands: Commands,
    mut event_queue: ResMut<EventQueue>,
    mut game_state: ResMut<GameState>,
    mut station_occupancy: ResMut<StationOccupancy>,
    station_positions: Res<StationPositions>,
    sprite_assets: Res<SpriteAssets>,
    mut agents: Query<(Entity, &Agent, &mut Transform, Option<&mut CurrentStation>), Without<MainAgent>>,
    mut main_agent: Query<(Entity, &mut Transform, &mut CurrentStation), With<MainAgent>>,
) {
    // Process only a few events per frame to allow rendering between spawn/despawn
    let mut events_processed = 0;
    const MAX_EVENTS_PER_FRAME: usize = 3;

    while let Some(event) = event_queue.pop() {
        events_processed += 1;
        if events_processed > MAX_EVENTS_PER_FRAME {
            // Re-queue the event for next frame
            event_queue.events.push_front(event);
            break;
        }
        match event.event_type {
            EventType::PreToolUse => {
                let station_type = StationType::for_tool(&event.tool_name);
                let base_pos = station_positions.get(station_type);

                // If it's a Task tool, spawn a new subagent
                if event.tool_name == "Task" {
                    if let Some(subagent_type_str) = event.get_subagent_type() {
                        let agent_type = AgentType::from_str(&subagent_type_str);

                        // Spawn at the agent's home station (based on agent type)
                        let home_station = agent_type.home_station();
                        let home_pos = station_positions.get(home_station);

                        // Calculate stagger index for labels
                        let stagger_index = station_occupancy.count_at_station(home_station);

                        // Get sprite for this agent type
                        if let Some(image_handle) = sprite_assets.agents.get(&agent_type) {
                            // Spawn at home station directly
                            let entity = commands
                                .spawn((
                                    Sprite {
                                        image: image_handle.clone(),
                                        custom_size: Some(Vec2::new(48.0, 48.0)),
                                        ..default()
                                    },
                                    Transform::from_xyz(home_pos.x, home_pos.y, 10.0),
                                    Agent {
                                        id: event.tool_use_id.clone(),
                                        agent_type,
                                        tool_use_id: Some(event.tool_use_id.clone()),
                                    },
                                    CurrentStation { station: Some(home_station) },
                                    LabelStagger { index: stagger_index },
                                    Speed(180.0),
                                    AnimationController::default(),
                                ))
                                .with_children(|parent| {
                                    // Name label - staggered vertically based on index
                                    let label_color = agent_type.color();
                                    let label_y_offset = 28.0 + (stagger_index as f32 * 16.0);

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
                                        Transform::from_xyz(0.0, label_y_offset, 0.5),
                                        AgentLabel,
                                    ));
                                    parent.spawn((
                                        Text2d::new(agent_type.label()),
                                        TextFont {
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                        Transform::from_xyz(0.0, label_y_offset, 1.0),
                                        AgentLabel,
                                    ));
                                })
                                .id();

                            // Register with station occupancy
                            station_occupancy.add_agent(home_station, entity);
                            game_state.agent_count += 1;
                        }
                    }
                } else {
                    // Move main agent to the appropriate station
                    if let Ok((entity, _transform, mut current_station)) = main_agent.get_single_mut() {
                        // Update station occupancy
                        if current_station.station.is_some() {
                            station_occupancy.remove_agent(entity);
                        }
                        station_occupancy.add_agent(station_type, entity);
                        current_station.station = Some(station_type);

                        // Calculate orbital offset
                        let orbital_offset = station_occupancy.get_orbital_offset(station_type, entity);
                        let target_pos = base_pos + orbital_offset;

                        commands.entity(entity).insert(MovementTarget {
                            position: target_pos,
                            station_type: Some(station_type),
                        });
                    }
                }
            }
            EventType::PostToolUse => {
                // Tool completed - despawn the subagent
                if event.tool_name == "Task" {
                    for (entity, agent, _transform, _) in agents.iter_mut() {
                        if agent.tool_use_id.as_ref() == Some(&event.tool_use_id) {
                            station_occupancy.remove_agent(entity);
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

/// System to clear all subagents when session changes
pub fn handle_session_change_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut station_occupancy: ResMut<StationOccupancy>,
    agents: Query<Entity, (With<Agent>, Without<MainAgent>)>,
) {
    if !game_state.session_changed {
        return;
    }

    // Clear the flag
    game_state.session_changed = false;

    // Despawn all subagents
    let mut count = 0;
    for entity in agents.iter() {
        station_occupancy.remove_agent(entity);
        commands.entity(entity).despawn_recursive();
        count += 1;
    }

    if count > 0 {
        game_state.agent_count = 1; // Just main agent remains
    }
}

/// System to update orbital positions when multiple agents are at the same station
pub fn update_orbital_positions_system(
    station_occupancy: Res<StationOccupancy>,
    station_positions: Res<StationPositions>,
    mut agents: Query<(Entity, &CurrentStation, &mut MovementTarget), (With<Agent>, Without<MainAgent>)>,
) {
    // Only run if station occupancy changed
    if !station_occupancy.is_changed() {
        return;
    }

    for (entity, current_station, mut target) in agents.iter_mut() {
        if let Some(station) = current_station.station {
            let base_pos = station_positions.get(station);
            let orbital_offset = station_occupancy.get_orbital_offset(station, entity);
            target.position = base_pos + orbital_offset;
        }
    }
}

/// System plugin for agent management
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationPositions>()
            .init_resource::<StationOccupancy>()
            .add_systems(
                Startup,
                spawn_main_agent.after(crate::sprites::generate_sprites),
            )
            .add_systems(Update, (
                handle_session_change_system,
                process_events_system.after(handle_session_change_system),
                update_orbital_positions_system.after(process_events_system),
            ));
    }
}
