mod components;
mod events;
mod resources;
mod sprites;
mod systems;

use bevy::prelude::*;

use components::*;
use resources::StationPositions;
use sprites::{SpriteAssets, SpriteGenPlugin};
use systems::{
    AgentPlugin, AnimationPlugin, EventReaderPlugin, MovementPlugin, UIPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Claude Code Visualiser".to_string(),
                resolution: (900.0, 700.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())) // Pixel art filtering
        // Background color
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.12)))
        // Our plugins
        .add_plugins((
            SpriteGenPlugin,
            EventReaderPlugin,
            AgentPlugin,
            MovementPlugin,
            AnimationPlugin,
            UIPlugin,
        ))
        // Setup systems
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_workspace.after(sprites::generate_sprites))
        .run();
}

/// Set up the 2D camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Set up the workspace with stations
pub fn setup_workspace(
    mut commands: Commands,
    station_positions: Res<StationPositions>,
    sprite_assets: Res<SpriteAssets>,
) {
    // Background workspace area - darker floor
    commands.spawn((
        Sprite {
            color: Color::srgba(0.15, 0.15, 0.2, 0.8),
            custom_size: Some(Vec2::new(750.0, 550.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Grid pattern on floor
    for i in -5..6 {
        // Vertical lines
        commands.spawn((
            Sprite {
                color: Color::srgba(0.25, 0.25, 0.3, 0.3),
                custom_size: Some(Vec2::new(2.0, 550.0)),
                ..default()
            },
            Transform::from_xyz(i as f32 * 70.0, 0.0, 0.1),
        ));
        // Horizontal lines
        commands.spawn((
            Sprite {
                color: Color::srgba(0.25, 0.25, 0.3, 0.3),
                custom_size: Some(Vec2::new(750.0, 2.0)),
                ..default()
            },
            Transform::from_xyz(0.0, i as f32 * 55.0, 0.1),
        ));
    }

    // Spawn stations with pixel art sprites
    spawn_station(&mut commands, &station_positions, &sprite_assets, StationType::Library);
    spawn_station(&mut commands, &station_positions, &sprite_assets, StationType::Desk);
    spawn_station(&mut commands, &station_positions, &sprite_assets, StationType::Terminal);
    spawn_station(&mut commands, &station_positions, &sprite_assets, StationType::WebPortal);
    spawn_station(&mut commands, &station_positions, &sprite_assets, StationType::MeetingArea);
}

/// Spawn a workstation with pixel art
fn spawn_station(
    commands: &mut Commands,
    positions: &StationPositions,
    sprite_assets: &SpriteAssets,
    station_type: StationType,
) {
    let pos = positions.get(station_type);

    // Get the sprite handle for this station
    if let Some(image_handle) = sprite_assets.stations.get(&station_type) {
        commands
            .spawn((
                Sprite {
                    image: image_handle.clone(),
                    custom_size: Some(Vec2::new(96.0, 72.0)), // Scale up 1.5x
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 1.0),
                Station { station_type },
            ))
            .with_children(|parent| {
                // Station label with background
                parent.spawn((
                    Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                        custom_size: Some(Vec2::new(70.0, 20.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, -50.0, 1.5),
                ));
                parent.spawn((
                    Text2d::new(station_type.label()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, -50.0, 2.0),
                ));
            });
    }
}
