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

fn check_add_point(points: &mut Vec<Vec2>) {
    if is_mouse_button_released(MouseButton::Left) {
        let pos = mouse_position();
        points.push(Vec2::new(pos.0, pos.1));
        println!("{}:{}", pos.0, pos.1);
    }
}

fn move_player_character(mut player: Player, sounds: &HashMap<&str, audio::Sound>) -> Player {
    if is_key_down(KeyCode::Right) {
        player.x += 2.0;
    }
    if is_key_down(KeyCode::Left) {
        player.x -= 2.0;
    } 

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

fn draw_map(map: Vec<Vec<String>>) {
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

#[macroquad::main("BasicShapes")]
async fn main() {
    let texture = load_texture("assets/pictures/grigor.png").await.unwrap();

    let soundAtlas = HashMap::from([
        ("jump", load_sound("assets/cartoon-jump.wav").await.unwrap()),
        ("background", load_sound("assets/monkey-music.wav").await.unwrap())
    ]);

    let mut ground_level = window::screen_height() - 100.0;

    let mut player = Player{x: 100.0, y: ground_level, jump: -1};
    let mut clicked_points = Vec::new();

    play_sound(*soundAtlas.get("background").unwrap(), PlaySoundParams{looped: true, volume: 0.1});
    let map = read_map();

    loop {
        clear_background(LIGHTGRAY);
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        player = move_player_character(player, &soundAtlas);
        
        if player.y >= ground_level && player.jump == -1 {
            player.jump = 0;
        }

        check_add_point(&mut clicked_points);

        for point in &clicked_points {
            draw_circle(point.x, point.y, 10.0, RED);
        }

        ground_level = window::screen_height() - 60.0;
        draw_map(map.clone());

        draw_texture(texture, player.x, player.y, WHITE);
        // draw_line(0.0, ground_level, window::screen_width(), ground_level, 1.0, GREEN);
        next_frame().await
    }
}