mod shape;
mod bullet;
mod bullet_vector;
mod enemy_square;
mod enemy_vector;
mod hero_circle;

use macroquad::prelude::*;
use crate::bullet_vector::BulletVector;
use crate::hero_circle::HeroCircle;
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
    let mut bullet_vector: BulletVector = BulletVector::new();
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
            if is_key_pressed(KeyCode::Space) {
                bullet_vector.fire(&circle.shape.x, &circle.shape.y);
            }

            // move squares down the screen
            enemy_vector.move_enemies(delta_time);
            // move bullets
            bullet_vector.move_bullets(delta_time);


            // COLLISION DETECTION
            if enemy_vector.collides_with(circle.clone()) {
                is_gameover = true;
            }

            enemy_vector.collides_with_bullets(&mut bullet_vector);

            enemy_vector.hide_enemies();
            bullet_vector.hide_bullets();

            // DRAW
            enemy_vector.draw_enemies();
            bullet_vector.draw_bullets();
            circle.draw();
        } else {
            if is_key_pressed(KeyCode::Space) {
                enemy_vector.clear();
                bullet_vector.clear();                
                circle = HeroCircle::new(screen_center_x, screen_center_y, MOVEMENT_SPEED);
                is_gameover = false;
            }
            else {
                set_game_over(screen_center_x, screen_center_y);
            }
        }

        // waits until the next frame is available
        next_frame().await
    }
}

fn set_game_over(x : f32, y : f32) {
    let text = "GAME OVER!";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        x - text_dimensions.width / 2.0,
        y - text_dimensions.height / 2.0,
        50.0,
        RED,
    );
}
