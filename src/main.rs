use macroquad::{prelude::*};
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

#[macroquad::main("BasicShapes")]
async fn main() {
    let texture = load_texture("assets/grigor.png").await.unwrap();
    let mut x = 50.0;
    let mut y = 50.0;
    let mut clicked_points = Vec::new();
    
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

        draw_texture(texture, x, y, WHITE);
        next_frame().await
    }
}