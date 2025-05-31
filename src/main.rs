use macroquad::prelude::*;

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;
    
    let mut x = screen_center_x;
    let mut y = screen_center_y;
    
    let speed = 2.0;
    
    loop {
        clear_background(DARKPURPLE);
        
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
    
        draw_circle(x, y, 16.0, YELLOW);
        
        // waits until the next frame is available
        next_frame().await
    }
}
