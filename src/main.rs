use macroquad::prelude::*;

mod shape;
mod enemy_vector;

use crate::shape::HeroCircle;
use crate::enemy_vector::EnemyVector;

const MOVEMENT_SPEED: f32 = 200.0;

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;
    rand::srand(miniquad::date::now() as u64);

    let mut is_gameover = false;
    let enemy_colors = [GRAY, BEIGE, PINK, RED];

    let mut enemy_vector: EnemyVector = EnemyVector::new();
    let mut circle = HeroCircle::new(screen_center_x, screen_center_y, MOVEMENT_SPEED);

    loop {
        // UPDATE
        clear_background(DARKPURPLE);

        if !is_gameover {
            // time that passed since the last frame
            let delta_time = get_frame_time();

            circle.set_speed(MOVEMENT_SPEED * delta_time);

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                let color = rand::gen_range(0, enemy_colors.len());

                enemy_vector.spawn_enemy(size, enemy_colors[color]);
            }

            // move squares down the screen
            enemy_vector.move_enemies(delta_time);
            enemy_vector.hide_enemies();

            if is_key_down(KeyCode::Right) {
                circle.move_right();
            }
            if is_key_down(KeyCode::Left) {
                circle.move_left();
            }
            if is_key_down(KeyCode::Down) {
                circle.move_down();
            }
            if is_key_down(KeyCode::Up) {
                circle.move_up();
            }

            // COLLISION DETECTION
            if enemy_vector.collides_with(&circle) {
                is_gameover = true;
            }

            // DRAW
            circle.draw();
            enemy_vector.draw_enemies();
        } else {
            if is_key_pressed(KeyCode::Space) {
                enemy_vector.clear();
                circle = HeroCircle::new(screen_center_x, screen_center_y, MOVEMENT_SPEED);
                is_gameover = false;
            }
            else {
                let text = "GAME OVER!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_center_x - text_dimensions.width / 2.0,
                    screen_center_y - text_dimensions.height / 2.0,
                    50.0,
                    RED,
                );
            }
        }

        // waits until the next frame is available
        next_frame().await
    }
}
