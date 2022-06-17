extern crate csv;

use macroquad::{prelude::*};
use soloud::*;
use std::{vec::Vec};

fn check_add_point(points: &mut Vec<Vec2>) {
    if is_mouse_button_released(MouseButton::Left) {
        let pos = mouse_position();
        points.push(Vec2::new(pos.0, pos.1));
    }
}

fn move_player_character(mut x: f32, mut y:  f32) -> (f32, f32) {
    if is_key_down(KeyCode::Right) {
        x += 2.0;
    }
    if is_key_down(KeyCode::Left) {
        x -= 2.0;
    } 
    if is_key_down(KeyCode::Up) {
        y -= 2.0;
    } 
    if is_key_down(KeyCode::Down) {
        y += 2.0;
    }
    return (x, y);
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
    let background_sound = Soloud::default().unwrap();  
    let mut wav_background = audio::Wav::default();
    let mut x = 50.0;
    let mut y = 50.0;
    let mut clicked_points = Vec::new();
    wav_background.load(&std::path::Path::new("assets/music/monkey-music.mp3")).unwrap();
    background_sound.play(&wav_background);

    let map = read_map();
    
    loop {
        clear_background(LIGHTGRAY);
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        (x, y) = move_player_character(x, y);


        check_add_point(&mut clicked_points);

        for point in &clicked_points {
            draw_circle(point.x, point.y, 10.0, RED);
        }

        draw_map(map.clone());

        draw_texture(texture, x, y, WHITE);
        next_frame().await
    }
}