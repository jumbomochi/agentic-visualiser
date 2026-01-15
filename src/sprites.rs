use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::seq::SliceRandom;

use crate::components::{AgentType, StationType};

/// Type alias for character drawing functions
type DrawFn = fn(&mut [u8], u32);

/// All available character drawing functions with their names
const CHARACTER_POOL: &[(DrawFn, &str)] = &[
    // Classic/2000s
    (draw_naruto, "Naruto"),
    (draw_luffy, "Luffy"),
    (draw_goku, "Goku"),
    (draw_sailor_moon, "Sailor Moon"),
    (draw_pikachu, "Pikachu"),
    (draw_edward_elric, "Edward Elric"),
    (draw_l, "L"),
    (draw_ichigo, "Ichigo"),
    (draw_lelouch, "Lelouch"),
    // 2010s
    (draw_kakashi, "Kakashi"),
    (draw_deku, "Deku"),
    (draw_totoro, "Totoro"),
    (draw_eren, "Eren"),
    (draw_levi, "Levi"),
    (draw_saitama, "Saitama"),
    (draw_kaneki, "Kaneki"),
    (draw_killua, "Killua"),
    (draw_kirito, "Kirito"),
    // 2020s
    (draw_tanjiro, "Tanjiro"),
    (draw_anya, "Anya"),
    (draw_denji, "Denji"),
    (draw_power, "Power"),
    (draw_frieren, "Frieren"),
    (draw_senku, "Senku"),
    (draw_mob, "Mob"),
    (draw_zero_two, "Zero Two"),
    // Supporting
    (draw_bulma, "Bulma"),
    (draw_conan, "Conan"),
    (draw_erwin, "Erwin"),
];

/// Resource holding generated sprite handles
#[derive(Resource, Default)]
pub struct SpriteAssets {
    pub agents: std::collections::HashMap<AgentType, Handle<Image>>,
    pub stations: std::collections::HashMap<StationType, Handle<Image>>,
}

/// Generate all sprite assets with randomized character assignments
pub fn generate_sprites(
    mut images: ResMut<Assets<Image>>,
    mut sprite_assets: ResMut<SpriteAssets>,
) {
    // Shuffle character assignments for this session
    let mut rng = rand::thread_rng();
    let mut characters: Vec<(DrawFn, &str)> = CHARACTER_POOL.to_vec();
    characters.shuffle(&mut rng);

    // Agent types to assign characters to
    let agent_types = [
        AgentType::Main,
        AgentType::Explore,
        AgentType::Plan,
        AgentType::Bash,
        AgentType::CodeReviewer,
        AgentType::UIUXReviewer,
        AgentType::StatuslineSetup,
        AgentType::ClaudeCodeGuide,
        AgentType::Haiku,
        AgentType::DevopsEngineer,
        AgentType::SecurityAnalyst,
        AgentType::ProjectManager,
        AgentType::General,
    ];

    // Log the random assignments for this session
    println!("\n🎲 Random character assignments for this session:");
    println!("================================================");

    // Generate agent sprites with shuffled characters
    for (i, agent_type) in agent_types.iter().enumerate() {
        let (draw_fn, char_name) = characters[i % characters.len()];

        println!("  {:?} → {}", agent_type, char_name);

        let size = 32u32;
        let mut pixels = vec![0u8; (size * size * 4) as usize];
        draw_fn(&mut pixels, size);
        let image = create_image(size, pixels);

        let handle = images.add(image);
        sprite_assets.agents.insert(*agent_type, handle);
    }
    println!("================================================\n");

    // Generate station sprites (these stay fixed)
    for station_type in [
        StationType::Library,
        StationType::Desk,
        StationType::Terminal,
        StationType::WebPortal,
        StationType::MeetingArea,
        StationType::Center,
    ] {
        let image = generate_station_sprite(station_type);
        let handle = images.add(image);
        sprite_assets.stations.insert(station_type, handle);
    }
}

/// Naruto - Main Agent (spiky blonde hair, orange outfit, whiskers)
fn draw_naruto(pixels: &mut [u8], size: u32) {
    let blonde = [255u8, 220, 80, 255];
    let orange = [255u8, 140, 40, 255];
    let blue = [30u8, 60, 140, 255]; // headband
    let skin = [255u8, 210, 170, 255];
    let whisker = [80u8, 60, 50, 255];

    // Spiky hair
    for &(x, y) in &[
        (15, 2), (16, 2), (17, 2),
        (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3),
        (10, 4), (11, 4), (12, 4), (13, 4), (18, 4), (19, 4), (20, 4), (21, 4),
        (9, 5), (10, 5), (11, 5), (20, 5), (21, 5), (22, 5),
        (8, 6), (9, 6), (22, 6), (23, 6),
    ] {
        set_pixel(pixels, size, x, y, blonde);
    }
    // Hair body
    for y in 4..10 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }

    // Headband (blue with metal plate)
    for x in 10..22 {
        set_pixel(pixels, size, x, 9, blue);
        set_pixel(pixels, size, x, 10, blue);
    }
    // Metal plate
    for x in 13..19 {
        set_pixel(pixels, size, x, 9, [180, 180, 190, 255]);
        set_pixel(pixels, size, x, 10, [180, 180, 190, 255]);
    }
    // Leaf symbol
    set_pixel(pixels, size, 15, 9, [60, 100, 60, 255]);
    set_pixel(pixels, size, 16, 9, [60, 100, 60, 255]);

    // Face
    for y in 11..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Eyes (big blue anime eyes)
    for y in 12..15 {
        set_pixel(pixels, size, 12, y, [30, 100, 200, 255]);
        set_pixel(pixels, size, 13, y, [30, 100, 200, 255]);
        set_pixel(pixels, size, 18, y, [30, 100, 200, 255]);
        set_pixel(pixels, size, 19, y, [30, 100, 200, 255]);
    }
    // Eye shine
    set_pixel(pixels, size, 12, 12, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 12, [255, 255, 255, 255]);

    // Whisker marks
    for y in [14, 15, 16] {
        set_pixel(pixels, size, 10, y, whisker);
        set_pixel(pixels, size, 21, y, whisker);
    }

    // Mouth (big grin)
    for x in 14..18 {
        set_pixel(pixels, size, x, 16, [200, 80, 80, 255]);
    }

    // Orange jacket
    for y in 18..28 {
        let width = if y < 21 { 10 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width) {
            set_pixel(pixels, size, x, y, orange);
        }
    }
    // Jacket collar (blue)
    for x in 13..19 {
        set_pixel(pixels, size, x, 18, blue);
    }
    // Zipper
    for y in 19..27 {
        set_pixel(pixels, size, 15, y, [200, 200, 200, 255]);
        set_pixel(pixels, size, 16, y, [200, 200, 200, 255]);
    }

    // Hands
    for y in 22..26 {
        set_pixel(pixels, size, 7, y, skin);
        set_pixel(pixels, size, 8, y, skin);
        set_pixel(pixels, size, 23, y, skin);
        set_pixel(pixels, size, 24, y, skin);
    }

    // Pants (blue)
    for y in 28..31 {
        for x in 11..15 {
            set_pixel(pixels, size, x, y, blue);
        }
        for x in 17..21 {
            set_pixel(pixels, size, x, y, blue);
        }
    }

    add_outline(pixels, size, [40, 30, 20, 255]);
}

/// Luffy - Explore Agent (straw hat, red vest, scar)
fn draw_luffy(pixels: &mut [u8], size: u32) {
    let straw = [240u8, 220, 150, 255];
    let straw_dark = [200u8, 170, 100, 255];
    let red = [200u8, 30, 30, 255];
    let skin = [255u8, 200, 160, 255];
    let black = [30u8, 30, 30, 255];

    // Straw hat
    for y in 1..5 {
        let width = 8 + y * 2;
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, straw);
        }
    }
    // Hat brim
    for x in 6..26 {
        set_pixel(pixels, size, x, 5, straw);
        set_pixel(pixels, size, x, 6, straw_dark);
    }
    // Hat band (red)
    for x in 11..21 {
        set_pixel(pixels, size, x, 4, red);
    }

    // Messy black hair under hat
    for y in 6..10 {
        for x in 10..22 {
            if (x + y) % 3 != 0 {
                set_pixel(pixels, size, x, y, black);
            }
        }
    }
    // Side hair
    for y in 7..12 {
        set_pixel(pixels, size, 9, y, black);
        set_pixel(pixels, size, 10, y, black);
        set_pixel(pixels, size, 21, y, black);
        set_pixel(pixels, size, 22, y, black);
    }

    // Face
    for y in 10..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Big round eyes
    for y in 11..15 {
        for x in 12..15 {
            set_pixel(pixels, size, x, y, black);
        }
        for x in 17..20 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Eye shine
    set_pixel(pixels, size, 13, 12, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 12, [255, 255, 255, 255]);

    // Scar under left eye
    set_pixel(pixels, size, 12, 15, [180, 80, 80, 255]);
    set_pixel(pixels, size, 13, 15, [180, 80, 80, 255]);

    // Big smile
    for x in 13..19 {
        set_pixel(pixels, size, x, 16, [200, 80, 80, 255]);
    }
    set_pixel(pixels, size, 12, 15, [200, 80, 80, 255]);
    set_pixel(pixels, size, 19, 15, [200, 80, 80, 255]);

    // Red vest (open)
    for y in 18..27 {
        // Left side
        for x in 9..14 {
            set_pixel(pixels, size, x, y, red);
        }
        // Right side
        for x in 18..23 {
            set_pixel(pixels, size, x, y, red);
        }
    }
    // Chest (visible)
    for y in 19..26 {
        for x in 14..18 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Arms
    for y in 20..26 {
        set_pixel(pixels, size, 7, y, skin);
        set_pixel(pixels, size, 8, y, skin);
        set_pixel(pixels, size, 24, y, skin);
        set_pixel(pixels, size, 25, y, skin);
    }

    // Blue shorts
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, [50, 80, 150, 255]);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, [50, 80, 150, 255]);
        }
    }

    add_outline(pixels, size, [40, 30, 20, 255]);
}

/// L from Death Note - Plan Agent (messy black hair, white shirt, hunched)
fn draw_l(pixels: &mut [u8], size: u32) {
    let black = [20u8, 20, 25, 255];
    let white = [250u8, 250, 250, 255];
    let skin = [245u8, 230, 220, 255]; // pale
    let eye_shadow = [180u8, 180, 190, 255];

    // Messy black hair
    for y in 2..12 {
        for x in 8..24 {
            // Messy pattern
            if ((x * 3 + y * 7) % 5 != 0) || y < 6 {
                set_pixel(pixels, size, x, y, black);
            }
        }
    }
    // Extra messy strands
    for &(x, y) in &[(7, 8), (7, 9), (6, 10), (24, 7), (25, 8), (25, 9), (24, 10)] {
        set_pixel(pixels, size, x, y, black);
    }

    // Pale face
    for y in 9..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Dark circles under eyes
    for x in 11..15 {
        set_pixel(pixels, size, x, 14, eye_shadow);
    }
    for x in 17..21 {
        set_pixel(pixels, size, x, 14, eye_shadow);
    }

    // Wide intense eyes
    for y in 11..14 {
        for x in 11..15 {
            set_pixel(pixels, size, x, y, [10, 10, 10, 255]);
        }
        for x in 17..21 {
            set_pixel(pixels, size, x, y, [10, 10, 10, 255]);
        }
    }
    // Intense stare (white reflection)
    set_pixel(pixels, size, 12, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 11, [255, 255, 255, 255]);

    // Small thoughtful mouth
    set_pixel(pixels, size, 15, 16, [180, 140, 140, 255]);
    set_pixel(pixels, size, 16, 16, [180, 140, 140, 255]);

    // White long-sleeve shirt (baggy)
    for y in 18..29 {
        let width = if y < 22 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Hunched posture - hands near face (thinking pose)
    for y in 16..20 {
        set_pixel(pixels, size, 10, y, skin);
        set_pixel(pixels, size, 21, y, skin);
    }

    // Blue jeans
    for y in 29..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, [60, 80, 120, 255]);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, [60, 80, 120, 255]);
        }
    }

    add_outline(pixels, size, [30, 30, 35, 255]);
}

/// Goku - Bash Agent (spiky black hair, orange gi)
fn draw_goku(pixels: &mut [u8], size: u32) {
    let black = [20u8, 20, 20, 255];
    let orange = [255u8, 140, 30, 255];
    let blue = [40u8, 80, 160, 255];
    let skin = [255u8, 200, 150, 255];

    // Super spiky hair
    let hair_points = [
        (14, 1), (15, 1), (16, 1), (17, 1),
        (12, 2), (13, 2), (18, 2), (19, 2),
        (10, 3), (11, 3), (20, 3), (21, 3),
        (8, 4), (9, 4), (22, 4), (23, 4),
        (7, 5), (8, 5), (23, 5), (24, 5),
        (6, 6), (7, 6), (24, 6), (25, 6),
        (7, 7), (24, 7),
    ];
    for &(x, y) in &hair_points {
        set_pixel(pixels, size, x, y, black);
    }
    // Hair body
    for y in 3..11 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    // Face
    for y in 10..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Determined eyes
    for y in 12..15 {
        set_pixel(pixels, size, 12, y, [20, 20, 20, 255]);
        set_pixel(pixels, size, 13, y, [20, 20, 20, 255]);
        set_pixel(pixels, size, 18, y, [20, 20, 20, 255]);
        set_pixel(pixels, size, 19, y, [20, 20, 20, 255]);
    }
    set_pixel(pixels, size, 12, 12, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 12, [255, 255, 255, 255]);

    // Confident smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 16, [200, 100, 100, 255]);
    }

    // Orange gi top
    for y in 18..27 {
        let width = if y < 21 { 10 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, orange);
        }
    }
    // Blue undershirt visible at collar
    for x in 13..19 {
        set_pixel(pixels, size, x, 18, blue);
    }
    // Belt
    for x in 10..22 {
        set_pixel(pixels, size, x, 26, blue);
    }

    // Muscular arms
    for y in 20..26 {
        set_pixel(pixels, size, 7, y, skin);
        set_pixel(pixels, size, 8, y, skin);
        set_pixel(pixels, size, 9, y, orange);
        set_pixel(pixels, size, 22, y, orange);
        set_pixel(pixels, size, 23, y, skin);
        set_pixel(pixels, size, 24, y, skin);
    }

    // Orange pants
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, orange);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, orange);
        }
    }

    add_outline(pixels, size, [30, 20, 10, 255]);
}

/// Kakashi - Code Reviewer (silver hair, mask, headband over eye)
fn draw_kakashi(pixels: &mut [u8], size: u32) {
    let silver = [200u8, 205, 215, 255];
    let navy = [30u8, 40, 60, 255];
    let mask = [40u8, 45, 55, 255];
    let skin = [245u8, 220, 200, 255];
    let red = [180u8, 40, 40, 255]; // Sharingan

    // Spiky silver hair (gravity defying)
    for y in 1..6 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, silver);
        }
    }
    // Extra spiky
    for &(x, y) in &[
        (8, 3), (9, 2), (10, 1), (21, 1), (22, 2), (23, 3),
        (7, 4), (8, 4), (23, 4), (24, 4),
        (9, 5), (22, 5),
    ] {
        set_pixel(pixels, size, x, y, silver);
    }
    // Hair covering headband
    for y in 6..10 {
        for x in 9..23 {
            if (x + y) % 2 == 0 {
                set_pixel(pixels, size, x, y, silver);
            }
        }
    }

    // Headband (slanted, covering left eye)
    for x in 9..22 {
        let y = 8 + (x as i32 - 15).abs() as u32 / 4;
        set_pixel(pixels, size, x, y, navy);
        set_pixel(pixels, size, x, y + 1, navy);
    }
    // Metal plate
    for x in 12..19 {
        set_pixel(pixels, size, x, 9, [160, 165, 175, 255]);
    }

    // Face (only right side visible)
    for y in 10..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Visible right eye (normal)
    for y in 11..14 {
        set_pixel(pixels, size, 17, y, [20, 20, 20, 255]);
        set_pixel(pixels, size, 18, y, [20, 20, 20, 255]);
    }
    set_pixel(pixels, size, 17, 11, [255, 255, 255, 255]);

    // Left eye covered/Sharingan peeking
    set_pixel(pixels, size, 13, 12, red);
    set_pixel(pixels, size, 13, 13, red);

    // Mask covering lower face
    for y in 14..20 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, mask);
        }
    }

    // Jonin vest (green)
    for y in 19..28 {
        let width = if y < 22 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, [80, 100, 80, 255]);
        }
    }
    // Vest pockets
    for y in 22..26 {
        set_pixel(pixels, size, 11, y, [60, 80, 60, 255]);
        set_pixel(pixels, size, 12, y, [60, 80, 60, 255]);
        set_pixel(pixels, size, 19, y, [60, 80, 60, 255]);
        set_pixel(pixels, size, 20, y, [60, 80, 60, 255]);
    }

    // Navy sleeves
    for y in 20..26 {
        set_pixel(pixels, size, 7, y, navy);
        set_pixel(pixels, size, 8, y, navy);
        set_pixel(pixels, size, 23, y, navy);
        set_pixel(pixels, size, 24, y, navy);
    }

    // Navy pants
    for y in 28..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, navy);
        }
    }

    add_outline(pixels, size, [25, 30, 40, 255]);
}

/// Sailor Moon - UI/UX Reviewer (twin tails, tiara, sailor outfit)
fn draw_sailor_moon(pixels: &mut [u8], size: u32) {
    let blonde = [255u8, 240, 100, 255];
    let pink = [255u8, 180, 200, 255];
    let blue = [50u8, 80, 180, 255];
    let white = [255u8, 255, 255, 255];
    let red = [220u8, 50, 50, 255];
    let skin = [255u8, 225, 200, 255];

    // Twin tail buns
    for y in 2..6 {
        for x in 6..11 {
            set_pixel(pixels, size, x, y, blonde);
        }
        for x in 21..26 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }
    // Long twin tails going down
    for y in 6..28 {
        set_pixel(pixels, size, 5, y, blonde);
        set_pixel(pixels, size, 6, y, blonde);
        set_pixel(pixels, size, 7, y, blonde);
        set_pixel(pixels, size, 24, y, blonde);
        set_pixel(pixels, size, 25, y, blonde);
        set_pixel(pixels, size, 26, y, blonde);
    }

    // Bangs
    for y in 4..9 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }
    // Fringe detail
    for x in 11..21 {
        set_pixel(pixels, size, x, 8, blonde);
    }

    // Tiara
    for x in 12..20 {
        set_pixel(pixels, size, x, 7, [255, 215, 0, 255]);
    }
    // Gem
    set_pixel(pixels, size, 15, 7, red);
    set_pixel(pixels, size, 16, 7, red);

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Big sparkly eyes
    for y in 10..14 {
        set_pixel(pixels, size, 12, y, blue);
        set_pixel(pixels, size, 13, y, blue);
        set_pixel(pixels, size, 18, y, blue);
        set_pixel(pixels, size, 19, y, blue);
    }
    // Eye shine (multiple)
    set_pixel(pixels, size, 12, 10, white);
    set_pixel(pixels, size, 13, 11, white);
    set_pixel(pixels, size, 18, 10, white);
    set_pixel(pixels, size, 19, 11, white);

    // Cute smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, pink);
    }

    // Sailor collar (blue with white stripes)
    for y in 17..22 {
        for x in 8..24 {
            if y < 19 || x < 12 || x > 19 {
                set_pixel(pixels, size, x, y, blue);
            }
        }
    }
    // White stripes on collar
    set_pixel(pixels, size, 9, 19, white);
    set_pixel(pixels, size, 10, 20, white);
    set_pixel(pixels, size, 22, 19, white);
    set_pixel(pixels, size, 21, 20, white);

    // White body suit
    for y in 19..27 {
        for x in 12..20 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Red bow
    for x in 13..19 {
        set_pixel(pixels, size, x, 18, red);
        set_pixel(pixels, size, x, 19, red);
    }
    // Bow center gem
    set_pixel(pixels, size, 15, 18, [255, 215, 0, 255]);
    set_pixel(pixels, size, 16, 18, [255, 215, 0, 255]);

    // Blue skirt
    for y in 27..31 {
        let width = 12 + (y - 27) * 2;
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, blue);
        }
    }

    add_outline(pixels, size, [40, 30, 50, 255]);
}

/// Pikachu - General Agent (yellow, red cheeks, pointy ears)
fn draw_pikachu(pixels: &mut [u8], size: u32) {
    let yellow = [255u8, 220, 50, 255];
    let yellow_dark = [220u8, 180, 30, 255];
    let red = [220u8, 60, 60, 255];
    let black = [20u8, 20, 20, 255];
    let brown = [100u8, 60, 30, 255];

    // Pointy ears
    for y in 1..8 {
        // Left ear
        let ear_width = (8 - y) / 2 + 1;
        for x in (8 - ear_width as u32)..(8 + 1) {
            set_pixel(pixels, size, x, y, yellow);
        }
        // Black ear tips
        if y < 4 {
            set_pixel(pixels, size, 8 - y / 2, y, black);
        }
        // Right ear
        for x in 23..(23 + ear_width as u32) {
            set_pixel(pixels, size, x, y, yellow);
        }
        if y < 4 {
            set_pixel(pixels, size, 23 + y / 2, y, black);
        }
    }

    // Head
    for y in 6..18 {
        let width = if y < 10 { 14 } else if y < 16 { 16 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, yellow);
        }
    }

    // Big round eyes
    for y in 9..14 {
        for x in 10..14 {
            set_pixel(pixels, size, x, y, black);
        }
        for x in 18..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Eye shine
    set_pixel(pixels, size, 11, 10, [255, 255, 255, 255]);
    set_pixel(pixels, size, 12, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 19, 10, [255, 255, 255, 255]);
    set_pixel(pixels, size, 20, 11, [255, 255, 255, 255]);

    // Red cheeks
    for y in 13..16 {
        for x in 8..11 {
            set_pixel(pixels, size, x, y, red);
        }
        for x in 21..24 {
            set_pixel(pixels, size, x, y, red);
        }
    }

    // Nose
    set_pixel(pixels, size, 15, 13, black);
    set_pixel(pixels, size, 16, 13, black);

    // Cute mouth
    set_pixel(pixels, size, 14, 15, black);
    set_pixel(pixels, size, 15, 16, black);
    set_pixel(pixels, size, 16, 16, black);
    set_pixel(pixels, size, 17, 15, black);

    // Body
    for y in 18..28 {
        let width = if y < 22 { 12 } else { 10 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, yellow);
        }
    }

    // Brown stripes on back
    for y in 19..23 {
        set_pixel(pixels, size, 11, y, brown);
        set_pixel(pixels, size, 20, y, brown);
    }

    // Arms
    for y in 20..25 {
        set_pixel(pixels, size, 8, y, yellow);
        set_pixel(pixels, size, 9, y, yellow);
        set_pixel(pixels, size, 22, y, yellow);
        set_pixel(pixels, size, 23, y, yellow);
    }

    // Feet
    for y in 28..31 {
        for x in 10..14 {
            set_pixel(pixels, size, x, y, yellow);
        }
        for x in 18..22 {
            set_pixel(pixels, size, x, y, yellow);
        }
    }

    // Lightning bolt tail (behind, to the right)
    let tail_points = [
        (24, 20), (25, 19), (26, 18), (27, 17),
        (26, 17), (25, 17), (26, 16), (27, 15),
        (28, 14), (27, 14), (26, 14),
    ];
    for &(x, y) in &tail_points {
        set_pixel(pixels, size, x, y, yellow_dark);
    }

    add_outline(pixels, size, [60, 50, 20, 255]);
}

/// Edward Elric - StatuslineSetup Agent (blonde braid, red coat, automail arm)
fn draw_edward_elric(pixels: &mut [u8], size: u32) {
    let blonde = [255u8, 220, 100, 255];
    let red = [180u8, 30, 30, 255];
    let black = [30u8, 30, 30, 255];
    let skin = [255u8, 210, 175, 255];
    let automail = [180u8, 190, 200, 255]; // silver metal

    // Blonde hair with antenna
    for &(x, y) in &[(15, 1), (16, 1), (14, 2), (15, 2), (16, 2), (17, 2)] {
        set_pixel(pixels, size, x, y, blonde);
    }
    // Main hair
    for y in 3..10 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }
    // Side hair spikes
    for y in 5..9 {
        set_pixel(pixels, size, 8, y, blonde);
        set_pixel(pixels, size, 9, y, blonde);
        set_pixel(pixels, size, 22, y, blonde);
        set_pixel(pixels, size, 23, y, blonde);
    }
    // Braid going down back
    for y in 10..22 {
        set_pixel(pixels, size, 21, y, blonde);
        set_pixel(pixels, size, 22, y, blonde);
        // Braid pattern
        if y % 3 == 0 {
            set_pixel(pixels, size, 21, y, [220, 180, 80, 255]);
        }
    }
    // Braid tie
    set_pixel(pixels, size, 21, 22, [60, 60, 60, 255]);
    set_pixel(pixels, size, 22, 22, [60, 60, 60, 255]);

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Determined golden eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [200, 170, 50, 255]);
        set_pixel(pixels, size, 13, y, [200, 170, 50, 255]);
        set_pixel(pixels, size, 17, y, [200, 170, 50, 255]);
        set_pixel(pixels, size, 18, y, [200, 170, 50, 255]);
    }
    // Eye pupils
    set_pixel(pixels, size, 13, 12, black);
    set_pixel(pixels, size, 18, 12, black);
    // Eye shine
    set_pixel(pixels, size, 12, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 17, 11, [255, 255, 255, 255]);

    // Confident smirk
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [200, 120, 120, 255]);
    }

    // Red coat (iconic)
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, red);
        }
    }
    // Coat flamel symbol (cross with snake)
    set_pixel(pixels, size, 15, 20, black);
    set_pixel(pixels, size, 16, 20, black);
    for y in 21..24 {
        set_pixel(pixels, size, 15, y, black);
        set_pixel(pixels, size, 16, y, black);
    }
    set_pixel(pixels, size, 14, 22, black);
    set_pixel(pixels, size, 17, 22, black);

    // Black shirt underneath
    for x in 13..19 {
        set_pixel(pixels, size, x, 17, black);
    }

    // Normal right arm
    for y in 19..25 {
        set_pixel(pixels, size, 7, y, red);
        set_pixel(pixels, size, 8, y, skin);
    }

    // Automail left arm (metal)
    for y in 19..26 {
        set_pixel(pixels, size, 23, y, automail);
        set_pixel(pixels, size, 24, y, automail);
        // Metal details
        if y % 2 == 0 {
            set_pixel(pixels, size, 23, y, [150, 160, 170, 255]);
        }
    }

    // Black pants
    for y in 28..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, black);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [50, 20, 20, 255]);
}

/// Deku (Izuku Midoriya) - ClaudeCodeGuide Agent (green curly hair, hero costume)
fn draw_deku(pixels: &mut [u8], size: u32) {
    let green = [60u8, 140, 80, 255];
    let dark_green = [40u8, 100, 60, 255];
    let skin = [255u8, 215, 185, 255];
    let white = [250u8, 250, 250, 255];
    let red = [200u8, 50, 50, 255];
    let black = [30u8, 30, 30, 255];

    // Messy curly green hair
    for y in 2..11 {
        for x in 9..23 {
            // Curly pattern
            if ((x + y) % 2 == 0) || y < 7 {
                set_pixel(pixels, size, x, y, green);
            }
        }
    }
    // Extra messy curls
    for &(x, y) in &[
        (7, 4), (8, 3), (8, 5), (7, 6),
        (23, 4), (24, 3), (24, 5), (23, 6),
        (10, 2), (12, 1), (19, 1), (21, 2),
    ] {
        set_pixel(pixels, size, x, y, green);
    }

    // Face with freckles
    for y in 10..18 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Freckles on cheeks
    for &(x, y) in &[(11, 14), (12, 15), (11, 16), (19, 14), (20, 15), (19, 16)] {
        set_pixel(pixels, size, x, y, [220, 180, 150, 255]);
    }

    // Big determined green eyes
    for y in 11..15 {
        set_pixel(pixels, size, 12, y, dark_green);
        set_pixel(pixels, size, 13, y, dark_green);
        set_pixel(pixels, size, 18, y, dark_green);
        set_pixel(pixels, size, 19, y, dark_green);
    }
    // Eye shine (big sparkly)
    set_pixel(pixels, size, 12, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 13, 12, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 19, 12, [255, 255, 255, 255]);

    // Determined smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 16, [200, 120, 120, 255]);
    }

    // Hero costume - green jumpsuit with white details
    for y in 18..28 {
        let width = if y < 21 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, dark_green);
        }
    }

    // White lines on costume
    for y in 19..26 {
        set_pixel(pixels, size, 15, y, white);
        set_pixel(pixels, size, 16, y, white);
    }

    // Red high-top sneakers (signature)
    for y in 28..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, red);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, red);
        }
    }
    // White shoe soles
    for x in 10..15 {
        set_pixel(pixels, size, x, 30, white);
    }
    for x in 17..22 {
        set_pixel(pixels, size, x, 30, white);
    }

    // Gloved hands
    for y in 21..26 {
        set_pixel(pixels, size, 7, y, red);
        set_pixel(pixels, size, 8, y, red);
        set_pixel(pixels, size, 23, y, red);
        set_pixel(pixels, size, 24, y, red);
    }

    // Mask/hood elements on head (rabbit ear-like)
    set_pixel(pixels, size, 10, 3, white);
    set_pixel(pixels, size, 11, 2, white);
    set_pixel(pixels, size, 20, 2, white);
    set_pixel(pixels, size, 21, 3, white);

    add_outline(pixels, size, [30, 60, 40, 255]);
}

/// Totoro - Haiku Agent (gray fluffy body, big smile, leaf on head)
fn draw_totoro(pixels: &mut [u8], size: u32) {
    let gray = [130u8, 135, 140, 255];
    let light_gray = [180u8, 185, 190, 255];
    let white = [250u8, 250, 250, 255];
    let black = [30u8, 30, 30, 255];
    let green = [80u8, 140, 80, 255];

    // Ears (pointy on top)
    for y in 1..7 {
        // Left ear
        let ear_width = (7 - y) / 2 + 1;
        for x in (10 - ear_width as u32)..(10 + 1) {
            set_pixel(pixels, size, x, y, gray);
        }
        // Right ear
        for x in 21..(21 + ear_width as u32 + 1) {
            set_pixel(pixels, size, x, y, gray);
        }
    }

    // Round body/head (Totoro is very round)
    for y in 6..28 {
        let width = if y < 10 {
            12 + (y - 6) * 2
        } else if y < 24 {
            20
        } else {
            20 - (y - 24) * 2
        };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, gray);
        }
    }

    // White belly
    for y in 14..26 {
        let width = if y < 18 { 8 + (y - 14) } else if y < 23 { 12 } else { 12 - (y - 23) * 2 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, light_gray);
        }
    }

    // Chest markings (chevron pattern)
    for i in 0..4 {
        let y = 15 + i * 2;
        let half_width = 2 + i;
        for x in (16 - half_width)..(16 + half_width) {
            set_pixel(pixels, size, x as u32, y as u32, gray);
        }
    }

    // Big round eyes
    for y in 9..14 {
        for x in 10..14 {
            set_pixel(pixels, size, x, y, white);
        }
        for x in 18..22 {
            set_pixel(pixels, size, x, y, white);
        }
    }
    // Pupils
    for y in 10..13 {
        set_pixel(pixels, size, 12, y, black);
        set_pixel(pixels, size, 13, y, black);
        set_pixel(pixels, size, 19, y, black);
        set_pixel(pixels, size, 20, y, black);
    }

    // Nose
    set_pixel(pixels, size, 15, 13, black);
    set_pixel(pixels, size, 16, 13, black);

    // Big Totoro grin
    for x in 11..21 {
        set_pixel(pixels, size, x, 15, black);
    }
    // Teeth hints
    for x in [12, 14, 16, 18] {
        set_pixel(pixels, size, x, 15, white);
    }

    // Whiskers
    for &(x, y) in &[
        (6, 11), (7, 12), (8, 13),
        (24, 11), (25, 12), (26, 13),
        (6, 13), (7, 13),
        (25, 13), (26, 13),
    ] {
        set_pixel(pixels, size, x, y, [100, 100, 100, 255]);
    }

    // Leaf on head (signature)
    for y in 2..6 {
        let leaf_width = if y < 4 { 5 - (4 - y) } else { 5 - (y - 3) };
        for x in (15 - leaf_width / 2)..(15 + leaf_width / 2 + 1) {
            set_pixel(pixels, size, x as u32, y, green);
        }
    }
    // Leaf stem
    set_pixel(pixels, size, 15, 6, [60, 100, 60, 255]);
    set_pixel(pixels, size, 15, 7, [60, 100, 60, 255]);

    // Small feet
    for y in 28..31 {
        for x in 11..15 {
            set_pixel(pixels, size, x, y, gray);
        }
        for x in 17..21 {
            set_pixel(pixels, size, x, y, gray);
        }
    }
    // Claws
    set_pixel(pixels, size, 11, 30, [80, 80, 80, 255]);
    set_pixel(pixels, size, 14, 30, [80, 80, 80, 255]);
    set_pixel(pixels, size, 17, 30, [80, 80, 80, 255]);
    set_pixel(pixels, size, 20, 30, [80, 80, 80, 255]);

    add_outline(pixels, size, [60, 65, 70, 255]);
}

/// Bulma - DevOps Engineer (blue/purple hair, tech outfit, capsule corp)
fn draw_bulma(pixels: &mut [u8], size: u32) {
    let blue_hair = [80u8, 180, 220, 255];
    let skin = [255u8, 220, 195, 255];
    let pink = [255u8, 150, 180, 255];
    let white = [250u8, 250, 250, 255];
    let red = [200u8, 50, 50, 255];

    // Short blue hair (classic Bulma style)
    for y in 2..10 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, blue_hair);
        }
    }
    // Hair bangs
    for x in 10..22 {
        set_pixel(pixels, size, x, 8, blue_hair);
        set_pixel(pixels, size, x, 9, blue_hair);
    }
    // Side hair
    for y in 8..14 {
        set_pixel(pixels, size, 8, y, blue_hair);
        set_pixel(pixels, size, 9, y, blue_hair);
        set_pixel(pixels, size, 22, y, blue_hair);
        set_pixel(pixels, size, 23, y, blue_hair);
    }

    // Face
    for y in 9..17 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Big blue eyes
    for y in 10..14 {
        set_pixel(pixels, size, 11, y, [50, 100, 200, 255]);
        set_pixel(pixels, size, 12, y, [50, 100, 200, 255]);
        set_pixel(pixels, size, 19, y, [50, 100, 200, 255]);
        set_pixel(pixels, size, 20, y, [50, 100, 200, 255]);
    }
    // Eye shine
    set_pixel(pixels, size, 11, 10, white);
    set_pixel(pixels, size, 19, 10, white);

    // Cute smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, pink);
    }

    // Pink dress/outfit
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, pink);
        }
    }

    // White collar
    for x in 12..20 {
        set_pixel(pixels, size, x, 17, white);
    }

    // Capsule Corp logo on chest (simplified)
    set_pixel(pixels, size, 15, 20, [200, 200, 50, 255]);
    set_pixel(pixels, size, 16, 20, [200, 200, 50, 255]);
    set_pixel(pixels, size, 15, 21, [200, 200, 50, 255]);
    set_pixel(pixels, size, 16, 21, [200, 200, 50, 255]);

    // Arms
    for y in 19..25 {
        set_pixel(pixels, size, 7, y, skin);
        set_pixel(pixels, size, 8, y, skin);
        set_pixel(pixels, size, 23, y, skin);
        set_pixel(pixels, size, 24, y, skin);
    }

    // Red boots
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, red);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, red);
        }
    }

    // Holding a capsule/gadget
    set_pixel(pixels, size, 6, 23, [100, 200, 100, 255]);
    set_pixel(pixels, size, 6, 24, [100, 200, 100, 255]);
    set_pixel(pixels, size, 7, 23, [100, 200, 100, 255]);
    set_pixel(pixels, size, 7, 24, [100, 200, 100, 255]);

    add_outline(pixels, size, [40, 80, 100, 255]);
}

/// Conan (Shinichi) - Security Analyst (glasses, bowtie, detective outfit)
fn draw_conan(pixels: &mut [u8], size: u32) {
    let black = [30u8, 30, 30, 255];
    let skin = [255u8, 215, 185, 255];
    let blue = [40u8, 60, 120, 255];
    let white = [250u8, 250, 250, 255];
    let red = [200u8, 50, 50, 255];

    // Spiky black hair
    for y in 2..10 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Hair spikes
    for &(x, y) in &[
        (9, 4), (8, 5), (9, 6),
        (22, 4), (23, 5), (22, 6),
        (12, 2), (14, 1), (17, 1), (19, 2),
    ] {
        set_pixel(pixels, size, x, y, black);
    }
    // Cowlick
    set_pixel(pixels, size, 16, 1, black);
    set_pixel(pixels, size, 15, 2, black);

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Glasses frames (iconic)
    for x in 10..15 {
        set_pixel(pixels, size, x, 10, black);
        set_pixel(pixels, size, x, 13, black);
    }
    set_pixel(pixels, size, 10, 11, black);
    set_pixel(pixels, size, 10, 12, black);
    set_pixel(pixels, size, 14, 11, black);
    set_pixel(pixels, size, 14, 12, black);

    for x in 17..22 {
        set_pixel(pixels, size, x, 10, black);
        set_pixel(pixels, size, x, 13, black);
    }
    set_pixel(pixels, size, 17, 11, black);
    set_pixel(pixels, size, 17, 12, black);
    set_pixel(pixels, size, 21, 11, black);
    set_pixel(pixels, size, 21, 12, black);

    // Bridge
    set_pixel(pixels, size, 15, 11, black);
    set_pixel(pixels, size, 16, 11, black);

    // Eyes behind glasses
    set_pixel(pixels, size, 12, 11, [30, 60, 100, 255]);
    set_pixel(pixels, size, 12, 12, [30, 60, 100, 255]);
    set_pixel(pixels, size, 19, 11, [30, 60, 100, 255]);
    set_pixel(pixels, size, 19, 12, [30, 60, 100, 255]);

    // Confident smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 100, 100, 255]);
    }

    // Blue suit jacket
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, blue);
        }
    }

    // White shirt visible
    for y in 17..21 {
        for x in 14..18 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Red bowtie (iconic)
    for x in 13..19 {
        set_pixel(pixels, size, x, 18, red);
    }
    set_pixel(pixels, size, 15, 17, red);
    set_pixel(pixels, size, 16, 17, red);
    set_pixel(pixels, size, 15, 19, red);
    set_pixel(pixels, size, 16, 19, red);

    // Blue shorts
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, blue);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, blue);
        }
    }

    // Sneakers
    for y in 29..31 {
        for x in 9..15 {
            set_pixel(pixels, size, x, y, white);
        }
        for x in 17..23 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    add_outline(pixels, size, [20, 30, 50, 255]);
}

/// Erwin Smith - Project Manager (blonde undercut, military uniform, cape)
fn draw_erwin(pixels: &mut [u8], size: u32) {
    let blonde = [220u8, 200, 140, 255];
    let skin = [255u8, 215, 190, 255];
    let brown = [100u8, 70, 50, 255];
    let tan = [180u8, 160, 120, 255];
    let white = [250u8, 250, 250, 255];
    let green = [80u8, 100, 80, 255]; // Survey Corps green

    // Blonde undercut hair
    for y in 2..9 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }
    // Parted/slicked style
    for y in 3..7 {
        set_pixel(pixels, size, 15, y, [200, 180, 120, 255]);
        set_pixel(pixels, size, 16, y, [200, 180, 120, 255]);
    }
    // Undercut sides (darker)
    for y in 7..10 {
        set_pixel(pixels, size, 9, y, brown);
        set_pixel(pixels, size, 10, y, brown);
        set_pixel(pixels, size, 21, y, brown);
        set_pixel(pixels, size, 22, y, brown);
    }

    // Face - strong jaw
    for y in 8..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Determined blue eyes
    for y in 10..13 {
        set_pixel(pixels, size, 12, y, [80, 130, 180, 255]);
        set_pixel(pixels, size, 13, y, [80, 130, 180, 255]);
        set_pixel(pixels, size, 18, y, [80, 130, 180, 255]);
        set_pixel(pixels, size, 19, y, [80, 130, 180, 255]);
    }
    // Thick eyebrows
    for x in 11..15 {
        set_pixel(pixels, size, x, 9, blonde);
    }
    for x in 17..21 {
        set_pixel(pixels, size, x, 9, blonde);
    }

    // Stern mouth
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 140, 140, 255]);
    }

    // Survey Corps jacket (tan/brown)
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, tan);
        }
    }

    // White shirt underneath
    for x in 14..18 {
        set_pixel(pixels, size, x, 17, white);
        set_pixel(pixels, size, x, 18, white);
    }

    // Brown leather straps (3DMG harness)
    for y in 19..26 {
        set_pixel(pixels, size, 12, y, brown);
        set_pixel(pixels, size, 19, y, brown);
    }
    // Horizontal strap
    for x in 12..20 {
        set_pixel(pixels, size, x, 22, brown);
    }

    // Wings of Freedom emblem on chest (simplified)
    set_pixel(pixels, size, 15, 20, white);
    set_pixel(pixels, size, 16, 20, [100, 150, 200, 255]);
    set_pixel(pixels, size, 14, 21, white);
    set_pixel(pixels, size, 17, 21, [100, 150, 200, 255]);

    // Green cape flowing behind
    for y in 18..28 {
        set_pixel(pixels, size, 6, y, green);
        set_pixel(pixels, size, 7, y, green);
        set_pixel(pixels, size, 24, y, green);
        set_pixel(pixels, size, 25, y, green);
    }
    // Cape bottom flutter
    for x in 5..8 {
        set_pixel(pixels, size, x, 28, green);
    }
    for x in 24..27 {
        set_pixel(pixels, size, x, 28, green);
    }

    // White pants
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, white);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Brown boots
    for y in 29..31 {
        for x in 9..15 {
            set_pixel(pixels, size, x, y, brown);
        }
        for x in 17..23 {
            set_pixel(pixels, size, x, y, brown);
        }
    }

    add_outline(pixels, size, [50, 40, 30, 255]);
}

/// Ichigo Kurosaki - Bleach (orange spiky hair, black shihakusho, Zangetsu)
fn draw_ichigo(pixels: &mut [u8], size: u32) {
    let orange = [255u8, 140, 50, 255];
    let black = [20u8, 20, 20, 255];
    let skin = [255u8, 210, 175, 255];
    let white = [250u8, 250, 250, 255];

    // Spiky orange hair
    for y in 1..10 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, orange);
        }
    }
    // Extra spiky
    for &(x, y) in &[(8, 3), (7, 4), (8, 5), (23, 3), (24, 4), (23, 5), (11, 1), (15, 0), (20, 1)] {
        set_pixel(pixels, size, x, y, orange);
    }

    // Face - intense expression
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Brown determined eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [100, 60, 30, 255]);
        set_pixel(pixels, size, 13, y, [100, 60, 30, 255]);
        set_pixel(pixels, size, 18, y, [100, 60, 30, 255]);
        set_pixel(pixels, size, 19, y, [100, 60, 30, 255]);
    }
    set_pixel(pixels, size, 12, 11, white);
    set_pixel(pixels, size, 18, 11, white);

    // Scowl
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 120, 120, 255]);
    }

    // Black shihakusho (soul reaper robe)
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, black);
        }
    }

    // White inner robe visible
    for x in 14..18 {
        set_pixel(pixels, size, x, 17, white);
        set_pixel(pixels, size, x, 18, white);
    }

    // Zangetsu (big sword) on back
    for y in 5..28 {
        set_pixel(pixels, size, 25, y, [60, 60, 70, 255]);
        set_pixel(pixels, size, 26, y, [60, 60, 70, 255]);
    }
    // Sword wrapping
    for y in [8, 12, 16, 20, 24] {
        set_pixel(pixels, size, 25, y, white);
        set_pixel(pixels, size, 26, y, white);
    }

    // Sandals
    for y in 28..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, [100, 80, 60, 255]);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, [100, 80, 60, 255]);
        }
    }

    add_outline(pixels, size, [40, 30, 20, 255]);
}

/// Lelouch - Code Geass (black hair, purple geass eye, royal cape)
fn draw_lelouch(pixels: &mut [u8], size: u32) {
    let black = [20u8, 20, 25, 255];
    let purple = [120u8, 50, 150, 255];
    let gold = [200u8, 170, 80, 255];
    let skin = [250u8, 230, 215, 255];
    let white = [250u8, 250, 250, 255];

    // Sleek black hair
    for y in 2..11 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Hair covering one eye slightly
    for y in 8..12 {
        set_pixel(pixels, size, 10, y, black);
        set_pixel(pixels, size, 11, y, black);
    }

    // Pale face
    for y in 10..17 {
        for x in 12..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // One normal purple eye, one Geass eye (red bird symbol)
    for y in 11..14 {
        set_pixel(pixels, size, 13, y, purple);
        set_pixel(pixels, size, 14, y, purple);
    }
    // Geass eye (glowing red)
    for y in 11..14 {
        set_pixel(pixels, size, 17, y, [220, 30, 30, 255]);
        set_pixel(pixels, size, 18, y, [220, 30, 30, 255]);
    }
    set_pixel(pixels, size, 17, 12, [255, 100, 100, 255]); // Glow

    // Confident smirk
    set_pixel(pixels, size, 15, 15, [180, 140, 140, 255]);
    set_pixel(pixels, size, 16, 15, [180, 140, 140, 255]);
    set_pixel(pixels, size, 17, 14, [180, 140, 140, 255]);

    // Royal outfit with gold trim
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, purple);
        }
    }
    // Gold trim
    for y in 17..26 {
        set_pixel(pixels, size, 10, y, gold);
        set_pixel(pixels, size, 21, y, gold);
    }
    // White cravat
    for x in 14..18 {
        set_pixel(pixels, size, x, 17, white);
        set_pixel(pixels, size, x, 18, white);
    }

    // Cape flowing behind
    for y in 16..28 {
        set_pixel(pixels, size, 6, y, [80, 30, 100, 255]);
        set_pixel(pixels, size, 7, y, [80, 30, 100, 255]);
        set_pixel(pixels, size, 24, y, [80, 30, 100, 255]);
        set_pixel(pixels, size, 25, y, [80, 30, 100, 255]);
    }

    // Black boots
    for y in 28..31 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [30, 20, 40, 255]);
}

/// Eren Jaeger - Attack on Titan (brown hair, green cape, angry eyes)
fn draw_eren(pixels: &mut [u8], size: u32) {
    let brown = [80u8, 60, 40, 255];
    let skin = [255u8, 210, 180, 255];
    let tan = [170u8, 150, 120, 255];
    let green = [80u8, 120, 80, 255];
    let white = [250u8, 250, 250, 255];

    // Messy brown hair
    for y in 2..10 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, brown);
        }
    }
    // Messy strands
    for &(x, y) in &[(8, 4), (7, 5), (8, 6), (23, 4), (24, 5), (23, 6), (12, 1), (19, 1)] {
        set_pixel(pixels, size, x, y, brown);
    }

    // Face - intense angry expression
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Green-gray eyes (Titan shifter)
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [80, 120, 100, 255]);
        set_pixel(pixels, size, 13, y, [80, 120, 100, 255]);
        set_pixel(pixels, size, 18, y, [80, 120, 100, 255]);
        set_pixel(pixels, size, 19, y, [80, 120, 100, 255]);
    }
    // Angry eyebrows
    for x in 11..15 {
        set_pixel(pixels, size, x, 10, brown);
    }
    for x in 17..21 {
        set_pixel(pixels, size, x, 10, brown);
    }

    // Determined grimace
    for x in 13..19 {
        set_pixel(pixels, size, x, 15, [160, 100, 100, 255]);
    }

    // Survey Corps jacket
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, tan);
        }
    }

    // Leather straps (3DMG harness)
    for y in 18..26 {
        set_pixel(pixels, size, 12, y, [60, 40, 30, 255]);
        set_pixel(pixels, size, 19, y, [60, 40, 30, 255]);
    }

    // Green cape
    for y in 17..28 {
        set_pixel(pixels, size, 6, y, green);
        set_pixel(pixels, size, 7, y, green);
        set_pixel(pixels, size, 24, y, green);
        set_pixel(pixels, size, 25, y, green);
    }

    // Wings of Freedom on back (blue/white)
    set_pixel(pixels, size, 15, 20, white);
    set_pixel(pixels, size, 16, 20, [100, 150, 200, 255]);

    // White pants
    for y in 27..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    add_outline(pixels, size, [40, 30, 20, 255]);
}

/// Levi Ackerman - Attack on Titan (black undercut, cravat, short but deadly)
fn draw_levi(pixels: &mut [u8], size: u32) {
    let black = [25u8, 25, 30, 255];
    let skin = [245u8, 225, 210, 255];
    let tan = [170u8, 150, 120, 255];
    let white = [250u8, 250, 250, 255];
    let green = [70u8, 100, 70, 255];

    // Black undercut hair
    for y in 4..10 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Undercut sides
    for y in 8..11 {
        set_pixel(pixels, size, 9, y, [40, 40, 45, 255]);
        set_pixel(pixels, size, 22, y, [40, 40, 45, 255]);
    }
    // Bangs over forehead
    for x in 11..21 {
        set_pixel(pixels, size, x, 8, black);
    }

    // Narrow stern face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Sharp narrow eyes
    for y in 11..13 {
        set_pixel(pixels, size, 12, y, [50, 60, 70, 255]);
        set_pixel(pixels, size, 13, y, [50, 60, 70, 255]);
        set_pixel(pixels, size, 18, y, [50, 60, 70, 255]);
        set_pixel(pixels, size, 19, y, [50, 60, 70, 255]);
    }

    // Unamused expression
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 150, 150, 255]);
    }

    // Survey Corps jacket
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, tan);
        }
    }

    // Signature white cravat
    for y in 17..20 {
        for x in 14..18 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Leather straps
    for y in 19..26 {
        set_pixel(pixels, size, 11, y, [50, 35, 25, 255]);
        set_pixel(pixels, size, 20, y, [50, 35, 25, 255]);
    }

    // Green cape
    for y in 17..27 {
        set_pixel(pixels, size, 6, y, green);
        set_pixel(pixels, size, 7, y, green);
        set_pixel(pixels, size, 24, y, green);
        set_pixel(pixels, size, 25, y, green);
    }

    // White pants
    for y in 27..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    add_outline(pixels, size, [20, 20, 25, 255]);
}

/// Saitama - One Punch Man (bald, yellow suit, red gloves, cape)
fn draw_saitama(pixels: &mut [u8], size: u32) {
    let skin = [255u8, 220, 190, 255];
    let yellow = [255u8, 220, 50, 255];
    let red = [200u8, 40, 40, 255];
    let white = [250u8, 250, 250, 255];
    let black = [30u8, 30, 30, 255];

    // Bald shiny head
    for y in 3..12 {
        let width = if y < 6 { 6 + (y - 3) * 2 } else { 12 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, skin);
        }
    }
    // Shine on head
    set_pixel(pixels, size, 13, 5, [255, 245, 230, 255]);
    set_pixel(pixels, size, 14, 4, [255, 245, 230, 255]);

    // Face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Simple dot eyes (iconic bored expression)
    set_pixel(pixels, size, 13, 12, black);
    set_pixel(pixels, size, 18, 12, black);

    // Simple line mouth
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 140, 140, 255]);
    }

    // Yellow hero suit
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, yellow);
        }
    }
    // Suit zipper
    for y in 18..26 {
        set_pixel(pixels, size, 15, y, [200, 180, 40, 255]);
        set_pixel(pixels, size, 16, y, [200, 180, 40, 255]);
    }

    // Red gloves
    for y in 22..26 {
        set_pixel(pixels, size, 7, y, red);
        set_pixel(pixels, size, 8, y, red);
        set_pixel(pixels, size, 23, y, red);
        set_pixel(pixels, size, 24, y, red);
    }

    // White cape
    for y in 17..29 {
        set_pixel(pixels, size, 5, y, white);
        set_pixel(pixels, size, 6, y, white);
        set_pixel(pixels, size, 25, y, white);
        set_pixel(pixels, size, 26, y, white);
    }

    // Red boots
    for y in 28..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, red);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, red);
        }
    }

    add_outline(pixels, size, [50, 40, 30, 255]);
}

/// Kaneki Ken - Tokyo Ghoul (white hair, mask, kagune hints)
fn draw_kaneki(pixels: &mut [u8], size: u32) {
    let white_hair = [240u8, 240, 245, 255];
    let skin = [250u8, 235, 225, 255];
    let black = [20u8, 20, 20, 255];
    let red = [180u8, 30, 30, 255];
    let mask = [30u8, 30, 35, 255];

    // White messy hair
    for y in 2..11 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, white_hair);
        }
    }
    // Messy strands
    for &(x, y) in &[(8, 4), (7, 6), (8, 7), (23, 4), (24, 6), (23, 7), (11, 1), (16, 1), (20, 1)] {
        set_pixel(pixels, size, x, y, white_hair);
    }

    // Face (half covered by mask)
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // One normal gray eye, one kakugan (red with black sclera)
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [80, 80, 90, 255]);
        set_pixel(pixels, size, 13, y, [80, 80, 90, 255]);
    }
    // Kakugan eye
    for y in 11..14 {
        set_pixel(pixels, size, 17, y, black); // Black sclera
        set_pixel(pixels, size, 18, y, black);
        set_pixel(pixels, size, 19, y, black);
    }
    set_pixel(pixels, size, 18, 12, red); // Red iris

    // Eyepatch/mask covering lower face
    for y in 14..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, mask);
        }
    }
    // Zipper mouth on mask
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [60, 60, 65, 255]);
    }

    // Black coat
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, black);
        }
    }

    // Kagune (red tentacles) behind
    for y in 18..26 {
        set_pixel(pixels, size, 5, y, red);
        set_pixel(pixels, size, 6, y, [200, 50, 50, 255]);
        set_pixel(pixels, size, 25, y, red);
        set_pixel(pixels, size, 26, y, [200, 50, 50, 255]);
    }
    // Kagune tips
    set_pixel(pixels, size, 4, 18, red);
    set_pixel(pixels, size, 27, 18, red);

    // Black pants
    for y in 28..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [15, 15, 20, 255]);
}

/// Killua Zoldyck - Hunter x Hunter (white spiky hair, blue outfit, assassin)
fn draw_killua(pixels: &mut [u8], size: u32) {
    let white_hair = [235u8, 240, 250, 255];
    let skin = [255u8, 225, 210, 255];
    let blue = [80u8, 120, 180, 255];
    let purple = [100u8, 80, 140, 255];
    let black = [30u8, 30, 35, 255];

    // Fluffy white spiky hair
    for y in 1..10 {
        for x in 8..24 {
            if (x + y) % 2 == 0 || y < 6 {
                set_pixel(pixels, size, x, y, white_hair);
            }
        }
    }
    // Extra spiky tufts
    for &(x, y) in &[(7, 3), (6, 5), (7, 6), (24, 3), (25, 5), (24, 6), (12, 0), (16, 0), (20, 0)] {
        set_pixel(pixels, size, x, y, white_hair);
    }

    // Youthful face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Cat-like blue eyes
    for y in 10..14 {
        set_pixel(pixels, size, 12, y, [60, 100, 180, 255]);
        set_pixel(pixels, size, 13, y, [60, 100, 180, 255]);
        set_pixel(pixels, size, 18, y, [60, 100, 180, 255]);
        set_pixel(pixels, size, 19, y, [60, 100, 180, 255]);
    }
    // Eye shine
    set_pixel(pixels, size, 12, 10, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 10, [255, 255, 255, 255]);

    // Playful smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [220, 150, 150, 255]);
    }

    // Blue turtleneck
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, blue);
        }
    }
    // Turtleneck collar
    for x in 13..19 {
        set_pixel(pixels, size, x, 17, [60, 100, 160, 255]);
    }

    // Purple shorts
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, purple);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, purple);
        }
    }

    // Skateboard shoes
    for y in 29..31 {
        for x in 9..15 {
            set_pixel(pixels, size, x, y, [80, 60, 120, 255]);
        }
        for x in 17..23 {
            set_pixel(pixels, size, x, y, [80, 60, 120, 255]);
        }
    }

    add_outline(pixels, size, [40, 50, 70, 255]);
}

/// Kirito - Sword Art Online (black hair, black coat, dual swords)
fn draw_kirito(pixels: &mut [u8], size: u32) {
    let black = [20u8, 20, 25, 255];
    let skin = [255u8, 215, 190, 255];
    let gray = [60u8, 60, 65, 255];
    let silver = [180u8, 185, 195, 255];

    // Black spiky hair
    for y in 2..11 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Spiky bangs
    for &(x, y) in &[(8, 5), (8, 6), (8, 7), (23, 5), (23, 6), (23, 7), (14, 1), (17, 1)] {
        set_pixel(pixels, size, x, y, black);
    }

    // Face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Dark determined eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [30, 30, 35, 255]);
        set_pixel(pixels, size, 13, y, [30, 30, 35, 255]);
        set_pixel(pixels, size, 18, y, [30, 30, 35, 255]);
        set_pixel(pixels, size, 19, y, [30, 30, 35, 255]);
    }
    set_pixel(pixels, size, 12, 11, [255, 255, 255, 255]);
    set_pixel(pixels, size, 18, 11, [255, 255, 255, 255]);

    // Calm expression
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 140, 140, 255]);
    }

    // Black coat
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Gray trim
    for y in 18..26 {
        set_pixel(pixels, size, 9, y, gray);
        set_pixel(pixels, size, 22, y, gray);
    }

    // Dual swords on back (crossed)
    // Elucidator (black)
    for y in 4..22 {
        set_pixel(pixels, size, 24, y, [40, 40, 50, 255]);
    }
    // Dark Repulser (light blue)
    for y in 6..24 {
        set_pixel(pixels, size, 26, y, [100, 180, 200, 255]);
    }
    // Hilts
    set_pixel(pixels, size, 24, 22, silver);
    set_pixel(pixels, size, 26, 24, [150, 200, 220, 255]);

    // Black boots
    for y in 28..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [15, 15, 20, 255]);
}

/// Tanjiro Kamado - Demon Slayer (checkered haori, scar, hanafuda earrings)
fn draw_tanjiro(pixels: &mut [u8], size: u32) {
    let burgundy = [120u8, 30, 40, 255];
    let black = [25u8, 25, 30, 255];
    let skin = [255u8, 215, 185, 255];
    let green = [60u8, 100, 80, 255];
    let white = [250u8, 250, 250, 255];

    // Dark burgundy/red hair
    for y in 2..10 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, burgundy);
        }
    }
    // Messy spiky hair
    for &(x, y) in &[(8, 4), (7, 5), (23, 4), (24, 5), (11, 1), (15, 1), (20, 1)] {
        set_pixel(pixels, size, x, y, burgundy);
    }

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Scar on forehead
    for x in 12..16 {
        set_pixel(pixels, size, x, 9, [180, 80, 80, 255]);
    }

    // Kind red-brown eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [140, 60, 60, 255]);
        set_pixel(pixels, size, 13, y, [140, 60, 60, 255]);
        set_pixel(pixels, size, 18, y, [140, 60, 60, 255]);
        set_pixel(pixels, size, 19, y, [140, 60, 60, 255]);
    }
    set_pixel(pixels, size, 12, 11, white);
    set_pixel(pixels, size, 18, 11, white);

    // Determined smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [200, 130, 130, 255]);
    }

    // Hanafuda earrings
    set_pixel(pixels, size, 10, 13, [220, 200, 180, 255]);
    set_pixel(pixels, size, 10, 14, [200, 50, 50, 255]);
    set_pixel(pixels, size, 21, 13, [220, 200, 180, 255]);
    set_pixel(pixels, size, 21, 14, [200, 50, 50, 255]);

    // Checkered haori (green and black pattern)
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            let checker = ((x + y) / 2) % 2 == 0;
            set_pixel(pixels, size, x, y, if checker { green } else { black });
        }
    }

    // White belt
    for x in 10..22 {
        set_pixel(pixels, size, x, 25, white);
    }

    // Black hakama pants
    for y in 26..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    // Nichirin sword on back
    for y in 6..24 {
        set_pixel(pixels, size, 25, y, [50, 50, 55, 255]);
    }

    add_outline(pixels, size, [40, 20, 25, 255]);
}

/// Anya Forger - Spy x Family (pink hair, green eyes, school uniform)
fn draw_anya(pixels: &mut [u8], size: u32) {
    let pink = [255u8, 180, 200, 255];
    let skin = [255u8, 225, 210, 255];
    let green = [80u8, 180, 120, 255];
    let black = [30u8, 30, 35, 255];
    let white = [250u8, 250, 250, 255];
    let red = [180u8, 50, 50, 255];

    // Pink hair with horn-like tufts
    for y in 3..11 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, pink);
        }
    }
    // Signature horn-like hair tufts
    for &(x, y) in &[(8, 3), (7, 4), (8, 5), (23, 3), (24, 4), (23, 5)] {
        set_pixel(pixels, size, x, y, pink);
    }
    // Top tufts
    set_pixel(pixels, size, 11, 2, pink);
    set_pixel(pixels, size, 12, 1, pink);
    set_pixel(pixels, size, 19, 2, pink);
    set_pixel(pixels, size, 20, 1, pink);

    // Cute round face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Big sparkly green eyes
    for y in 11..15 {
        set_pixel(pixels, size, 12, y, green);
        set_pixel(pixels, size, 13, y, green);
        set_pixel(pixels, size, 18, y, green);
        set_pixel(pixels, size, 19, y, green);
    }
    // Multiple eye sparkles (Anya style)
    set_pixel(pixels, size, 12, 11, white);
    set_pixel(pixels, size, 13, 12, white);
    set_pixel(pixels, size, 18, 11, white);
    set_pixel(pixels, size, 19, 12, white);

    // "Waku waku" smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [255, 150, 150, 255]);
    }
    // Blush
    set_pixel(pixels, size, 11, 14, [255, 180, 180, 255]);
    set_pixel(pixels, size, 20, 14, [255, 180, 180, 255]);

    // Eden Academy uniform - black dress
    for y in 17..27 {
        let width = if y < 20 { 10 } else { 12 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // White collar
    for x in 13..19 {
        set_pixel(pixels, size, x, 17, white);
    }
    // Red ribbon
    set_pixel(pixels, size, 15, 18, red);
    set_pixel(pixels, size, 16, 18, red);

    // White socks
    for y in 27..30 {
        for x in 11..14 {
            set_pixel(pixels, size, x, y, white);
        }
        for x in 18..21 {
            set_pixel(pixels, size, x, y, white);
        }
    }

    // Black shoes
    for x in 10..15 {
        set_pixel(pixels, size, x, 30, black);
    }
    for x in 17..22 {
        set_pixel(pixels, size, x, 30, black);
    }

    add_outline(pixels, size, [80, 50, 60, 255]);
}

/// Denji - Chainsaw Man (blonde messy hair, sharp teeth, chainsaw elements)
fn draw_denji(pixels: &mut [u8], size: u32) {
    let blonde = [230u8, 200, 130, 255];
    let skin = [255u8, 210, 175, 255];
    let red = [180u8, 40, 40, 255];
    let gray = [100u8, 100, 105, 255];
    let white = [250u8, 250, 250, 255];

    // Messy blonde hair
    for y in 2..10 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, blonde);
        }
    }
    // Very messy spikes
    for &(x, y) in &[(7, 3), (6, 5), (8, 6), (24, 3), (25, 5), (23, 6), (11, 1), (14, 0), (18, 0), (21, 1)] {
        set_pixel(pixels, size, x, y, blonde);
    }

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Yellow eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, [200, 180, 60, 255]);
        set_pixel(pixels, size, 13, y, [200, 180, 60, 255]);
        set_pixel(pixels, size, 18, y, [200, 180, 60, 255]);
        set_pixel(pixels, size, 19, y, [200, 180, 60, 255]);
    }
    // Pupils
    set_pixel(pixels, size, 13, 12, [30, 30, 30, 255]);
    set_pixel(pixels, size, 19, 12, [30, 30, 30, 255]);

    // Shark-like grin with sharp teeth
    for x in 12..20 {
        set_pixel(pixels, size, x, 15, [200, 100, 100, 255]);
    }
    // Teeth
    for x in [13, 15, 17, 19] {
        set_pixel(pixels, size, x, 15, white);
    }

    // Chainsaw cord coming from chest
    set_pixel(pixels, size, 15, 17, red);
    set_pixel(pixels, size, 16, 17, red);

    // School uniform shirt (white, disheveled)
    for y in 17..26 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, white);
        }
    }
    // Loose tie
    for y in 18..24 {
        set_pixel(pixels, size, 15, y, red);
        set_pixel(pixels, size, 16, y, red);
    }

    // Dark pants
    for y in 26..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, gray);
        }
    }

    add_outline(pixels, size, [50, 40, 30, 255]);
}

/// Power - Chainsaw Man (pink hair with horns, sharp teeth, blood fiend)
fn draw_power(pixels: &mut [u8], size: u32) {
    let pink = [255u8, 180, 190, 255];
    let skin = [255u8, 225, 215, 255];
    let red = [180u8, 40, 40, 255];
    let yellow = [240u8, 220, 80, 255];
    let black = [30u8, 30, 35, 255];
    let white = [250u8, 250, 250, 255];

    // Long pink hair
    for y in 3..16 {
        for x in 7..25 {
            if y < 11 || x < 10 || x > 21 {
                set_pixel(pixels, size, x, y, pink);
            }
        }
    }

    // Red horns
    set_pixel(pixels, size, 10, 2, red);
    set_pixel(pixels, size, 11, 1, red);
    set_pixel(pixels, size, 11, 2, red);
    set_pixel(pixels, size, 20, 2, red);
    set_pixel(pixels, size, 21, 1, red);
    set_pixel(pixels, size, 21, 2, red);

    // Face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Yellow cat-like eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, yellow);
        set_pixel(pixels, size, 13, y, yellow);
        set_pixel(pixels, size, 18, y, yellow);
        set_pixel(pixels, size, 19, y, yellow);
    }
    // Slit pupils
    set_pixel(pixels, size, 13, 12, black);
    set_pixel(pixels, size, 19, 12, black);

    // Fanged smug grin
    for x in 13..19 {
        set_pixel(pixels, size, x, 15, [200, 120, 120, 255]);
    }
    // Fangs
    set_pixel(pixels, size, 13, 15, white);
    set_pixel(pixels, size, 18, 15, white);

    // Business shirt
    for y in 17..26 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, white);
        }
    }
    // Black tie
    for y in 18..25 {
        set_pixel(pixels, size, 15, y, black);
        set_pixel(pixels, size, 16, y, black);
    }

    // Dark skirt
    for y in 26..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [80, 50, 60, 255]);
}

/// Frieren - Frieren: Beyond Journey's End (silver-white hair, elf ears, green/white robes)
fn draw_frieren(pixels: &mut [u8], size: u32) {
    let silver = [230u8, 235, 245, 255];
    let skin = [255u8, 240, 230, 255];
    let green = [60u8, 100, 80, 255];
    let white = [250u8, 250, 250, 255];
    let purple = [140u8, 100, 160, 255];

    // Long silver-white hair
    for y in 2..20 {
        for x in 6..26 {
            if y < 10 || x < 9 || x > 22 {
                set_pixel(pixels, size, x, y, silver);
            }
        }
    }
    // Twin tails at sides
    for y in 10..24 {
        set_pixel(pixels, size, 5, y, silver);
        set_pixel(pixels, size, 6, y, silver);
        set_pixel(pixels, size, 25, y, silver);
        set_pixel(pixels, size, 26, y, silver);
    }

    // Elf ears (pointed, sticking out from hair)
    set_pixel(pixels, size, 7, 10, skin);
    set_pixel(pixels, size, 6, 11, skin);
    set_pixel(pixels, size, 5, 12, skin);
    set_pixel(pixels, size, 24, 10, skin);
    set_pixel(pixels, size, 25, 11, skin);
    set_pixel(pixels, size, 26, 12, skin);

    // Calm elfin face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Serene purple eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, purple);
        set_pixel(pixels, size, 13, y, purple);
        set_pixel(pixels, size, 18, y, purple);
        set_pixel(pixels, size, 19, y, purple);
    }
    set_pixel(pixels, size, 12, 11, white);
    set_pixel(pixels, size, 18, 11, white);

    // Subtle calm expression
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [200, 160, 160, 255]);
    }

    // White and green mage robes
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 16 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, white);
        }
    }
    // Green trim
    for y in 20..27 {
        set_pixel(pixels, size, 9, y, green);
        set_pixel(pixels, size, 10, y, green);
        set_pixel(pixels, size, 21, y, green);
        set_pixel(pixels, size, 22, y, green);
    }
    // Green collar/cape
    for x in 11..21 {
        set_pixel(pixels, size, x, 17, green);
    }

    // Simple boots
    for y in 28..31 {
        for x in 11..15 {
            set_pixel(pixels, size, x, y, [80, 70, 60, 255]);
        }
        for x in 17..21 {
            set_pixel(pixels, size, x, y, [80, 70, 60, 255]);
        }
    }

    add_outline(pixels, size, [80, 85, 95, 255]);
}

/// Senku Ishigami - Dr. Stone (green-tipped white hair, lab coat, genius)
fn draw_senku(pixels: &mut [u8], size: u32) {
    let white_hair = [240u8, 240, 235, 255];
    let green = [80u8, 160, 100, 255];
    let skin = [255u8, 220, 195, 255];
    let white = [250u8, 250, 250, 255];
    let black = [30u8, 30, 35, 255];
    let red = [180u8, 50, 50, 255];

    // Wild leek-shaped hair (white with green tips)
    for y in 0..10 {
        for x in 8..24 {
            let is_tip = y < 4;
            set_pixel(pixels, size, x, y, if is_tip { green } else { white_hair });
        }
    }
    // Extra wild spikes
    for &(x, y) in &[(7, 2), (6, 4), (24, 2), (25, 4), (15, 0), (16, 0)] {
        set_pixel(pixels, size, x, y, green);
    }

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Cunning red eyes
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, red);
        set_pixel(pixels, size, 13, y, red);
        set_pixel(pixels, size, 18, y, red);
        set_pixel(pixels, size, 19, y, red);
    }
    set_pixel(pixels, size, 12, 11, [255, 200, 200, 255]);
    set_pixel(pixels, size, 18, 11, [255, 200, 200, 255]);

    // Confident smirk
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [200, 130, 130, 255]);
    }
    set_pixel(pixels, size, 18, 14, [200, 130, 130, 255]);

    // Stone age outfit with lab coat vibe
    for y in 17..27 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, [180, 170, 150, 255]); // Rough cloth
        }
    }
    // E=mc² on chest (simplified)
    set_pixel(pixels, size, 14, 20, black);
    set_pixel(pixels, size, 15, 20, black);
    set_pixel(pixels, size, 16, 20, black);

    // Rope belt
    for x in 10..22 {
        set_pixel(pixels, size, x, 25, [120, 100, 70, 255]);
    }

    // Simple sandals
    for y in 27..31 {
        for x in 10..15 {
            set_pixel(pixels, size, x, y, [100, 80, 60, 255]);
        }
        for x in 17..22 {
            set_pixel(pixels, size, x, y, [100, 80, 60, 255]);
        }
    }

    add_outline(pixels, size, [50, 60, 50, 255]);
}

/// Mob (Shigeo Kageyama) - Mob Psycho 100 (black bowl cut, simple face, psychic aura)
fn draw_mob(pixels: &mut [u8], size: u32) {
    let black = [25u8, 25, 30, 255];
    let skin = [255u8, 220, 200, 255];
    let blue = [100u8, 150, 220, 255];
    let white = [250u8, 250, 250, 255];
    let purple = [150u8, 100, 200, 255]; // Psychic aura

    // Simple bowl cut
    for y in 3..11 {
        for x in 9..23 {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Rounded bowl shape
    for x in 10..22 {
        set_pixel(pixels, size, x, 10, black);
    }

    // Simple round face
    for y in 10..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Simple dot eyes (iconic Mob style)
    set_pixel(pixels, size, 13, 12, black);
    set_pixel(pixels, size, 18, 12, black);

    // Simple neutral expression
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [180, 150, 150, 255]);
    }

    // Gakuran (black school uniform)
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, black);
        }
    }
    // Gold buttons
    for y in [19, 22, 25] {
        set_pixel(pixels, size, 15, y, [200, 180, 80, 255]);
        set_pixel(pixels, size, 16, y, [200, 180, 80, 255]);
    }
    // White collar line
    for x in 13..19 {
        set_pixel(pixels, size, x, 17, white);
    }

    // Psychic aura (subtle purple glow around)
    for &(x, y) in &[(7, 10), (7, 15), (7, 20), (24, 10), (24, 15), (24, 20)] {
        set_pixel(pixels, size, x, y, purple);
    }

    // Black pants
    for y in 28..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, black);
        }
    }

    add_outline(pixels, size, [20, 20, 25, 255]);
}

/// Zero Two - Darling in the Franxx (pink hair, red horns, red pilot suit)
fn draw_zero_two(pixels: &mut [u8], size: u32) {
    let pink = [255u8, 150, 180, 255];
    let skin = [255u8, 225, 210, 255];
    let red = [200u8, 50, 60, 255];
    let white = [250u8, 250, 250, 255];
    let cyan = [100u8, 200, 200, 255];

    // Long pink hair
    for y in 2..18 {
        for x in 6..26 {
            if y < 10 || x < 9 || x > 22 {
                set_pixel(pixels, size, x, y, pink);
            }
        }
    }

    // Red horns
    set_pixel(pixels, size, 10, 1, red);
    set_pixel(pixels, size, 11, 0, red);
    set_pixel(pixels, size, 11, 1, red);
    set_pixel(pixels, size, 12, 2, red);
    set_pixel(pixels, size, 19, 2, red);
    set_pixel(pixels, size, 20, 0, red);
    set_pixel(pixels, size, 20, 1, red);
    set_pixel(pixels, size, 21, 1, red);

    // Face
    for y in 9..17 {
        for x in 11..21 {
            set_pixel(pixels, size, x, y, skin);
        }
    }

    // Cyan eyes with red rings
    for y in 11..14 {
        set_pixel(pixels, size, 12, y, cyan);
        set_pixel(pixels, size, 13, y, cyan);
        set_pixel(pixels, size, 18, y, cyan);
        set_pixel(pixels, size, 19, y, cyan);
    }
    // Red ring around eyes
    set_pixel(pixels, size, 11, 12, red);
    set_pixel(pixels, size, 14, 12, red);
    set_pixel(pixels, size, 17, 12, red);
    set_pixel(pixels, size, 20, 12, red);

    // Seductive smile
    for x in 14..18 {
        set_pixel(pixels, size, x, 15, [220, 130, 140, 255]);
    }

    // Red pilot suit
    for y in 17..28 {
        let width = if y < 20 { 12 } else { 14 };
        let start = 16 - width / 2;
        for x in start..(start + width as u32) {
            set_pixel(pixels, size, x, y, red);
        }
    }
    // White accents
    for y in 18..25 {
        set_pixel(pixels, size, 10, y, white);
        set_pixel(pixels, size, 21, y, white);
    }
    // Suit details
    for x in 14..18 {
        set_pixel(pixels, size, x, 20, white);
    }

    // Red boots
    for y in 28..31 {
        for x in 10..22 {
            set_pixel(pixels, size, x, y, red);
        }
    }

    add_outline(pixels, size, [100, 40, 50, 255]);
}

/// Generate a station sprite
fn generate_station_sprite(station_type: StationType) -> Image {
    let width = 64u32;
    let height = 48u32;
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    match station_type {
        StationType::Library => draw_library(&mut pixels, width, height),
        StationType::Desk => draw_desk(&mut pixels, width, height),
        StationType::Terminal => draw_terminal(&mut pixels, width, height),
        StationType::WebPortal => draw_web_portal(&mut pixels, width, height),
        StationType::MeetingArea => draw_meeting_area(&mut pixels, width, height),
        StationType::Center => {}
    }

    create_image_rect(width, height, pixels)
}

fn draw_library(pixels: &mut [u8], width: u32, height: u32) {
    let wood = [139u8, 90, 43, 255];
    let book_colors = [
        [200, 50, 50, 255],
        [50, 150, 50, 255],
        [50, 50, 200, 255],
        [200, 200, 50, 255],
        [150, 50, 150, 255],
    ];

    // Bookshelf frame
    for y in 5..height - 2 {
        for x in 5..width - 5 {
            if y > 5 && y < height - 3 && x > 6 && x < width - 6 {
                set_pixel_rect(pixels, width, x, y, [100, 70, 40, 255]);
            }
            if x == 5 || x == width - 6 || y == 5 || y == height - 3 {
                set_pixel_rect(pixels, width, x, y, wood);
            }
            if (y == 18 || y == 31) && x > 5 && x < width - 5 {
                set_pixel_rect(pixels, width, x, y, wood);
            }
        }
    }

    for shelf_y in [7u32, 20, 33] {
        let mut book_x = 8u32;
        for i in 0..8 {
            let color = book_colors[i % book_colors.len()];
            let book_height = 9 + (i % 3) as u32;
            for y in shelf_y..(shelf_y + book_height).min(shelf_y + 10) {
                for x in book_x..book_x + 5 {
                    set_pixel_rect(pixels, width, x, y, color);
                }
            }
            book_x += 7;
        }
    }
}

fn draw_desk(pixels: &mut [u8], width: u32, height: u32) {
    let wood = [180u8, 140, 100, 255];
    let paper = [250u8, 250, 245, 255];
    let pencil = [255u8, 220, 50, 255];

    for y in 15..25 {
        for x in 5..width - 5 {
            set_pixel_rect(pixels, width, x, y, wood);
        }
    }

    for y in 25..height - 2 {
        for x in 8..14 {
            set_pixel_rect(pixels, width, x, y, wood);
        }
        for x in (width - 14)..(width - 8) {
            set_pixel_rect(pixels, width, x, y, wood);
        }
    }

    for y in 8..14 {
        for x in 20..40 {
            set_pixel_rect(pixels, width, x, y, paper);
        }
    }

    for x in 42..52 {
        set_pixel_rect(pixels, width, x, 10, pencil);
        set_pixel_rect(pixels, width, x, 11, pencil);
    }
    set_pixel_rect(pixels, width, 52, 10, [200, 150, 100, 255]);
    set_pixel_rect(pixels, width, 52, 11, [200, 150, 100, 255]);
}

fn draw_terminal(pixels: &mut [u8], width: u32, height: u32) {
    let monitor_frame = [50u8, 50, 60, 255];
    let screen = [20u8, 30, 20, 255];
    let text_green = [0u8, 255, 100, 255];

    for y in 5..35 {
        for x in 10..width - 10 {
            set_pixel_rect(pixels, width, x, y, monitor_frame);
        }
    }

    for y in 8..32 {
        for x in 13..width - 13 {
            set_pixel_rect(pixels, width, x, y, screen);
        }
    }

    for (i, line_y) in [12u32, 16, 20, 24, 28].iter().enumerate() {
        let line_len = 20 + (i * 5) % 15;
        for x in 15..(15 + line_len as u32).min(width - 15) {
            set_pixel_rect(pixels, width, x, *line_y, text_green);
        }
    }

    set_pixel_rect(pixels, width, 16, 28, [0, 255, 100, 200]);
    set_pixel_rect(pixels, width, 17, 28, [0, 255, 100, 200]);

    for y in 35..40 {
        for x in 28..36 {
            set_pixel_rect(pixels, width, x, y, monitor_frame);
        }
    }
    for y in 40..height - 2 {
        for x in 22..42 {
            set_pixel_rect(pixels, width, x, y, monitor_frame);
        }
    }
}

fn draw_web_portal(pixels: &mut [u8], width: u32, height: u32) {
    let center_x = width / 2;
    let center_y = height / 2;

    for r in (5..20).rev() {
        let intensity = 255 - (r * 10) as u8;
        let color = [100, intensity, 255, intensity];

        for angle in 0..360 {
            let rad = (angle as f32) * std::f32::consts::PI / 180.0;
            let x = center_x as f32 + (r as f32 * rad.cos());
            let y = center_y as f32 + (r as f32 * rad.sin());
            if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
                set_pixel_rect(pixels, width, x as u32, y as u32, color);
            }
        }
    }

    for y in (center_y - 5)..(center_y + 5) {
        for x in (center_x - 5)..(center_x + 5) {
            let dx = (x as i32 - center_x as i32).abs();
            let dy = (y as i32 - center_y as i32).abs();
            if dx * dx + dy * dy < 20 {
                set_pixel_rect(pixels, width, x, y, [200, 255, 255, 255]);
            }
        }
    }

    let sparkle = [255u8, 255, 255, 255];
    set_pixel_rect(pixels, width, center_x - 15, center_y - 10, sparkle);
    set_pixel_rect(pixels, width, center_x + 12, center_y - 8, sparkle);
    set_pixel_rect(pixels, width, center_x - 10, center_y + 12, sparkle);
    set_pixel_rect(pixels, width, center_x + 14, center_y + 6, sparkle);
}

fn draw_meeting_area(pixels: &mut [u8], width: u32, height: u32) {
    let center_x = width / 2;
    let center_y = height / 2;
    let floor = [80u8, 80, 100, 255];
    let table = [160u8, 140, 120, 255];

    for y in 0..height {
        for x in 0..width {
            let dx = (x as i32 - center_x as i32) as f32;
            let dy = (y as i32 - center_y as i32) as f32 * 1.5;
            if dx * dx + dy * dy < 500.0 {
                set_pixel_rect(pixels, width, x, y, floor);
            }
        }
    }

    for y in (center_y - 8)..(center_y + 8) {
        for x in (center_x - 12)..(center_x + 12) {
            let dx = (x as i32 - center_x as i32) as f32;
            let dy = (y as i32 - center_y as i32) as f32;
            if dx * dx + dy * dy < 80.0 {
                set_pixel_rect(pixels, width, x, y, table);
            }
        }
    }
}

fn set_pixel(pixels: &mut [u8], size: u32, x: u32, y: u32, color: [u8; 4]) {
    if x < size && y < size {
        let idx = ((y * size + x) * 4) as usize;
        pixels[idx..idx + 4].copy_from_slice(&color);
    }
}

fn set_pixel_rect(pixels: &mut [u8], width: u32, x: u32, y: u32, color: [u8; 4]) {
    let idx = ((y * width + x) * 4) as usize;
    if idx + 4 <= pixels.len() {
        pixels[idx..idx + 4].copy_from_slice(&color);
    }
}

fn add_outline(pixels: &mut [u8], size: u32, outline_color: [u8; 4]) {
    let mut outline_pixels = Vec::new();

    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            if pixels[idx + 3] > 0 {
                for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                        let nidx = ((ny as u32 * size + nx as u32) * 4) as usize;
                        if pixels[nidx + 3] == 0 {
                            outline_pixels.push((nx as u32, ny as u32));
                        }
                    }
                }
            }
        }
    }

    for (x, y) in outline_pixels {
        set_pixel(pixels, size, x, y, outline_color);
    }
}

fn create_image(size: u32, pixels: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}

fn create_image_rect(width: u32, height: u32, pixels: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}

pub struct SpriteGenPlugin;

impl Plugin for SpriteGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, generate_sprites.before(super::setup_workspace));
    }
}
