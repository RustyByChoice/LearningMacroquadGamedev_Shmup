use macroquad::prelude::*;

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        // waits until the next frame is available
        next_frame().await
    }
}
