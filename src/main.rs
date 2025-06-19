mod shape;
mod bullet;
mod bullet_vector;
mod enemy_square;
mod enemy_vector;
mod hero_circle;
mod high_score;

use macroquad::prelude::*;
use crate::bullet_vector::BulletVector;
use crate::hero_circle::HeroCircle;
use crate::enemy_vector::EnemyVector;
use crate::high_score::HighScore;

const MOVEMENT_SPEED: f32 = 200.0;
const SHOT_FREQUENCY: f64 = 0.25;

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

    let mut high_score = HighScore::new();

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
                let shots_fired = get_time();
                // println!("{}", shots_fired);

                if shots_fired > bullet_vector.last_time_fired + SHOT_FREQUENCY {
                    bullet_vector.fire(&circle.shape.x, &circle.shape.y);
                    bullet_vector.last_time_fired = shots_fired;
                }
            }

            // move squares down the screen
            enemy_vector.move_enemies(delta_time);
            enemy_vector.hide_enemies();
            // move bullets
            bullet_vector.move_bullets(delta_time);
            bullet_vector.hide_bullets();

            // COLLISION DETECTION
            if enemy_vector.collides_with(circle.clone()) {
                high_score.save_high_score();
                // if score == high_score {
                //     fs::write("highscore.dat", high_score.to_string()).ok();
                // }
                is_gameover = true;
            }

            if enemy_vector.collides_with_bullets(&mut bullet_vector) {
                // score += square.size.round() as u32;
                high_score.add();
                // score += 1;

                // if high_score < score {
                //     high_score_toppled = true;
                // }

                // high_score = high_score.max(score);
            }

            // DRAW
            enemy_vector.draw_enemies();
            bullet_vector.draw_bullets();
            circle.draw();
            draw_high_score(&high_score);
        } else {
            if is_key_pressed(KeyCode::Space) {
                enemy_vector.clear();
                bullet_vector.clear();
                high_score.clear();
                circle = HeroCircle::new(screen_center_x, screen_center_y, MOVEMENT_SPEED);
                is_gameover = false;
            }
            else {
                set_game_over(screen_center_x, screen_center_y, &high_score);
            }
        }

        // waits until the next frame is available
        next_frame().await
    }
}

fn set_game_over(x : f32, y : f32, high_score: &HighScore) {
    let text = "GAME OVER!";
    let text_dimensions = measure_text(text, None, 50, 1.0);

    let caption_x: f32 = x - text_dimensions.width / 2.0;
    let caption_y: f32 = y - text_dimensions.height / 2.0;

    draw_text(
        text,
        caption_x,
        caption_y,
        50.0,
        RED,
    );

    if high_score.is_new_high() {
        let score_text = format!("Your new high score is: {}", high_score.get_current_high());
        let score_text_dimensions = measure_text(&score_text, None, 50, 1.0);
        draw_text(
            &score_text,
            x - score_text_dimensions.width / 2.0,
            caption_y + 50.0,
            50.0,
            RED,
        );
    }
}

fn draw_high_score(score: &HighScore) {
    draw_score(60.0, "Score", score.get_current_score());
    draw_score(35.0, "High_score", score.get_current_high());
}

fn draw_score(y:f32, caption : &str, score: u32) {
    let highscore_text = format!("{}: {}", caption, score);
    let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
    draw_text(
        &highscore_text.as_str(),
        screen_width() - text_dimensions.width - 10.0, 
        y,
        25.0,
        WHITE
    );
}