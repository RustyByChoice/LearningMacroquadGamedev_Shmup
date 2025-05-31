use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 200.0;

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;
    
    let mut x = screen_center_x;
    let mut y = screen_center_y;
    let radius = 16.0;
    
    let mut speed;
    
    loop {
        clear_background(DARKPURPLE);

        // time that passed since the last frame
        let delta_time = get_frame_time();

        speed = MOVEMENT_SPEED * delta_time;
        
        if is_key_down(KeyCode::Right) {
            x += speed;
        }
        if is_key_down(KeyCode::Left) {
            x -= speed;
        }
        if is_key_down(KeyCode::Down) {
            y += speed;
        }
        if is_key_down(KeyCode::Up) {
            y -= speed;
        }

        // clamp is used to clamp a value between a min and max value
        x = clamp(x, 0.0 + radius, screen_width() - radius);
        y = clamp(y, 0.0 + radius, screen_height() - radius);
        
        draw_circle(x, y, radius, YELLOW);
        
        // waits until the next frame is available
        next_frame().await
    }
}
