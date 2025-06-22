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
const FONT_SIZE : f32 = 50.0;
const FONT_SCALE : f32 = 1.0;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver
}

// tell Macroquad which function will be run when application starts, and what will be the window title
#[macroquad::main("My Shmup")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;

    let enemy_colors = [GRAY, BEIGE, PINK, RED];

    let mut enemy_vector: EnemyVector = EnemyVector::new();
    let mut bullet_vector: BulletVector = BulletVector::new();
    let mut circle = HeroCircle::new(get_center_x(), get_center_y(), MOVEMENT_SPEED);

    let mut high_score = HighScore::new();

    loop {
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    enemy_vector.clear();
                    bullet_vector.clear();
                    high_score.clear();
                    circle = HeroCircle::new(get_center_x(), get_center_y(), MOVEMENT_SPEED);
                    game_state = GameState::Playing;
                }

                put_text_in_center("Press space");
            }
            GameState::Playing => {
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

                    if shots_fired > bullet_vector.last_time_fired + SHOT_FREQUENCY {
                        bullet_vector.fire(&circle.shape.x, &circle.shape.y);
                        bullet_vector.last_time_fired = shots_fired;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
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
                    game_state = GameState::GameOver;
                }

                if enemy_vector.collides_with_bullets(&mut bullet_vector) {
                    high_score.add();
                }

                // DRAW
                enemy_vector.draw_enemies();
                bullet_vector.draw_bullets();
                circle.draw();
                draw_high_score(&high_score);                
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                put_text_in_center("Paused");
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                set_game_over(get_center_x(), get_center_y(), &high_score);                
            }
        }

        // waits until the next frame is available
        next_frame().await
    }
}

fn set_game_over(x : f32, y : f32, high_score: &HighScore) {
    let text = "GAME OVER!";
    let text_dimensions = measure_text(text, None, 50, FONT_SCALE);

    let caption_x: f32 = x - text_dimensions.width / 2.0;
    let caption_y: f32 = y - text_dimensions.height / 2.0;

    draw_text(
        text,
        caption_x,
        caption_y,
        FONT_SIZE,
        RED,
    );

    if high_score.is_new_high() {
        let score_text = format!("Your new high score is: {}", high_score.get_current_high());
        let score_text_dimensions = measure_text(&score_text, None, 50, FONT_SCALE);
        draw_text(
            &score_text,
            x - score_text_dimensions.width / 2.0,
            caption_y + FONT_SIZE,
            FONT_SIZE,
            RED,
        );
    }
}

fn draw_high_score(score: &HighScore) {
    draw_score(35.0, "High_score", score.get_current_high());
    draw_score(60.0, "Score", score.get_current_score());
}

fn draw_score(y:f32, caption : &str, score: u32) {
    let highscore_font_size : f32 = 25.0;
    let highscore_text = format!("{}: {}", caption, score);
    let text_dimensions = measure_text(&highscore_text, None, highscore_font_size as u16, FONT_SCALE);
    draw_text(
        &highscore_text,
        screen_width() - text_dimensions.width - 10.0, 
        y,
        highscore_font_size,
        WHITE
    );
}

fn put_text_in_center(text : &str) {
    let text_dimensions = measure_text(text, None, FONT_SIZE as u16, FONT_SCALE);
    draw_text(
        text, 
        get_center_x() - text_dimensions.width / 2.0, 
        get_center_y(), 
        FONT_SIZE, 
        WHITE
    );
}

fn get_center_x() -> f32 {
    screen_width() / 2.0   
}

fn get_center_y() -> f32 {
    screen_height() / 2.0   
}