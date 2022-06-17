extern crate csv;

use macroquad::{prelude::*, audio, audio::{load_sound, PlaySoundParams, play_sound, play_sound_once}, window};
// use soloud::*;
use std::{vec::Vec};
use std::collections::hash_map::HashMap;

struct Player {
    x: f32,
    y: f32,
    jump: i32
}

/**
 * Saves mouse position when clicking left mouse button into the passed vector.
 */
fn check_add_point(points: &mut Vec<Vec2>) {
    if is_mouse_button_released(MouseButton::Left) {
        let pos = mouse_position();
        points.push(Vec2::new(pos.0, pos.1));
        println!("{}:{}", pos.0, pos.1);
    }
}

/**
 * Controls player character movement, allows moving left, right and jumping. 
 * Also plays sound when jumping.
 */
fn move_player_character(mut player: Player, sounds: &HashMap<&str, audio::Sound>) -> Player {
    if is_key_down(KeyCode::Right) {
        player.x += 2.0;
    }
    if is_key_down(KeyCode::Left) {
        player.x -= 2.0;
    } 

    // Jump logic, hold down the space bar to jump higher, up to a maximum.
    // Then fall down until you hit the ground before you can jump again.
    if is_key_pressed(KeyCode::Space) && player.jump == 0 {
        play_sound_once(*sounds.get("jump").unwrap());
        player.jump = 1;
        player.y -= 2.0;
    } else if is_key_down(KeyCode::Space) && 0 < player.jump && player.jump < 40 {
        player.jump += 1;
        player.y -= 2.0;
    } else if is_key_released(KeyCode::Space) || player.jump >= 40 {
        player.jump = -1;
    }

    if player.jump == -1 {
        player.y += 2.0;
    }
    
    return player;
}

/**
 * Read the map file (scene.csv) and return it as a 2d array which can later be drawn as
 * square tiles.
 */
fn read_map() -> Vec<Vec<String>> {
    let mut rdr = csv::Reader::from_path("assets/scene.csv").unwrap();
    let mut map = vec![vec![String::from(""); 50]; 50];
    for (j, record) in rdr.records().enumerate() {
        for (i, tile) in record.unwrap().iter().enumerate() {
            map[i][j] = tile.to_string();
        }
    }
    return map;
}

/**
 * Render the map from the passed 2d array. 
 */
fn draw_map(map: &Vec<Vec<String>>) {
    let squares = 50 as f32;
    let square_size_width = screen_width() / squares;
    let square_size_height = screen_height() / squares;

    for i in 0..50 {
        for j in 0..50 {
            match map[j][i].as_str(){
                "T"=>draw_rectangle(j as f32*square_size_width, i as f32*square_size_height, square_size_width,square_size_height,BLUE),
                "G"=>draw_rectangle(j as f32*square_size_width, i as f32*square_size_height, square_size_width,square_size_height,GREEN),
                "A"=>draw_rectangle(j as f32*square_size_width, i as f32*square_size_height, square_size_width,square_size_height,LIGHTGRAY),
                _=>print!(""),
            }
        }
    }
}

/**
 * The main function, program starts here.
 */
#[macroquad::main("BasicShapes")]
async fn main() {
    // Fire we load some assets (textures, audio, etc).
    let texture = load_texture("assets/pictures/grigor.png").await.unwrap();

    let sound_atlas = HashMap::from([
        ("jump", load_sound("assets/cartoon-jump.wav").await.unwrap()),
        ("background", load_sound("assets/monkey-music.wav").await.unwrap())
    ]);

    // Then initialize some other things that needs to be preserved from frame to frame.
    let mut ground_level = window::screen_height() - 100.0;
    let mut player = Player{x: 100.0, y: ground_level, jump: -1};
    let mut clicked_points = Vec::new();

    // Start the background music. Ohyea!
    play_sound(*sound_atlas.get("background").unwrap(), PlaySoundParams{looped: true, volume: 0.1});
    let map = read_map();

    // The main loop. This runs 60 times a second (60 fps), and everything that should move
    // or change from one frame to the next must be updated inside this.
    loop {
        // Clear out the previous frame.
        clear_background(LIGHTGRAY);
        
        // Quit the program with escape.
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        // Update the player position and handle "collision" with the ground.
        player = move_player_character(player, &sound_atlas);
        if player.y >= ground_level && player.jump == -1 {
            player.jump = 0;
        }

        // Possibly draw circles where the player has clicked (does nothing right now...)
        check_add_point(&mut clicked_points);
        for point in &clicked_points {
            draw_circle(point.x, point.y, 10.0, RED);
        }

        ground_level = window::screen_height() - 60.0;
        // Draw the map and the player, then draw the buffer to screen.
        draw_map(&map);
        draw_texture(texture, player.x, player.y, WHITE);
        next_frame().await
    }
}