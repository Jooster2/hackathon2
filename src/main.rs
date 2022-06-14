use macroquad::{prelude::*};
use soloud::*;
use std::vec::Vec;

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

fn is_jumping() -> bool {
    if is_key_down(KeyCode::Space) {
        return true;
    }
    return false;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let texture = load_texture("assets/grigor.png").await.unwrap();
    let background_sound = Soloud::default().unwrap(); 
    let jump_sound = Soloud::default().unwrap(); 
    let mut wav_background = audio::Wav::default();
    let mut wav_jump = audio::Wav::default();
    let mut x = 50.0;
    let mut y = 50.0;
    let mut clicked_points = Vec::new();
    wav_background.load(&std::path::Path::new("assets/monkey-music.mp3")).unwrap();
    background_sound.play(&wav_background);
    wav_jump.load(&std::path::Path::new("assets/cartoon-jump.mp3")).unwrap();

    loop {
        clear_background(LIGHTGRAY);
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        (x, y) = move_player_character(x, y);

        if is_jumping() {
            jump_sound.play(&wav_jump);
        } 

        check_add_point(&mut clicked_points);

        for point in &clicked_points {
            draw_circle(point.x, point.y, 10.0, RED);
        }

        draw_texture(texture, x, y, WHITE);
        next_frame().await
    }
}