use macroquad::prelude::*;

mod shape;

use crate::shape::Shape;

const MOVEMENT_SPEED: f32 = 200.0;

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;
    rand::srand(miniquad::date::now() as u64);
    
    //let mut x = screen_center_x;
    //let mut y = screen_center_y;
    //let radius = 16.0;
    
    //let mut speed;
    
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_center_x,
        y: screen_center_y,
    };

    loop {
        // UPDATE
        clear_background(DARKPURPLE);

        // time that passed since the last frame
        let delta_time = get_frame_time();

        circle.speed = MOVEMENT_SPEED * delta_time;

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
               size,
               speed: rand::gen_range(50.0, 150.0),
               x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
               y: -size,
            });
        }
        
        // move squares down the screen
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        
        // remove squares outside of screen
        squares.retain(|square| square.y < screen_height() + square.size);
                             
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed;
        }

        // clamp is used to clamp a value between a min and max value
        circle.x = clamp(circle.x, 0.0 + circle.size, screen_width() - circle.size);
        circle.y = clamp(circle.y, 0.0 + circle.size, screen_height() - circle.size);
        
        // DRAW
        
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        // waits until the next frame is available
        next_frame().await
    }
}
