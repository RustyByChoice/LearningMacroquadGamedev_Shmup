mod shape;
mod bullet;
mod bullet_vector;
mod enemy_square;
mod enemy_vector;
mod hero_circle;
mod high_score;
mod caption;

use macroquad::prelude::*;
use crate::bullet_vector::BulletVector;
use crate::hero_circle::HeroCircle;
use crate::enemy_vector::EnemyVector;
use crate::high_score::HighScore;
use crate::caption::Caption;

const FRAGMENT_SHADER: &str = include_str!("shaders/starfield-shader.glsl");
const VERTEX_SHADER: &str = include_str!("shaders/vertex-shader.glsl");

const MOVEMENT_SPEED: f32 = 200.0;
const SHOT_FREQUENCY: f64 = 0.25;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver
}

#[macroquad::main("My Shmup")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl { 
            vertex: VERTEX_SHADER, 
            fragment: FRAGMENT_SHADER 
        }, MaterialParams { 
            //pipeline_params: (), 
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
            // textures: ()
        }).unwrap();

    let mut game_state = GameState::MainMenu;

    let mut enemy_vector: EnemyVector = EnemyVector::new();
    let mut bullet_vector: BulletVector = BulletVector::new();
    let mut circle = HeroCircle::new(get_center_x(), get_center_y(), MOVEMENT_SPEED);

    let mut high_score = HighScore::new();

    loop {
        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(&render_target.texture, 0., 0., WHITE, DrawTextureParams 
            { 
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default() 
                // source: (), 
                // rotation: (), 
                // flip_x: (), 
                // flip_y: (), 
                // pivot: () 
            });
        gl_use_default_material();

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

                let title = Caption::new(
                    "SHMUP'EM UP!".to_string(),
                    None,
                    Some(100.0),
                    None
                );

                put_text_in_center(Some(get_center_y() - title.get_dimensions().height), title);
                put_text_in_center(None, Caption::default("Press space"));
            }
            GameState::Playing => {
                // time that passed since the last frame
                let delta_time = get_frame_time();

                circle.set_speed(MOVEMENT_SPEED * delta_time);

                if rand::gen_range(0, 99) >= 95 {
                    enemy_vector.spawn_enemy();
                }

                if is_key_down(KeyCode::Right) {
                    circle.move_right();
                    direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    circle.move_left();
                    direction_modifier -= 0.05 * delta_time;
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
                put_text_in_center(None, Caption::default("Paused"));
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                set_game_over(&high_score);                
            }
        }

        // waits until the next frame is available
        next_frame().await
    }
}

fn set_game_over(high_score: &HighScore) {
    let x = get_center_x();
    let y = get_center_y();

    let text = Caption::new(
        "GAME OVER!".to_owned(), 
        Some(RED), 
        None, 
        None);

    let caption_y: f32 = y - text.get_dimensions().height / 2.0;

    draw_text(
        &text.text,
        x - text.get_dimensions().width / 2.0,
        caption_y,
        *&text.font_size,
        *&text.color
    );

    if high_score.is_new_high() {
        let score_text = Caption::new(
            format!("Your new high score is: {}", high_score.get_current_high()), 
            Some(RED), 
            None, 
            None);

        draw_text(
            &score_text.text,
            x - score_text.get_dimensions().width / 2.0,
            caption_y + &text.font_size,
            score_text.font_size,
            score_text.color,
        );
    }
}

fn draw_high_score(score: &HighScore) {
    let high_score = format!("High Score: {}", score.get_current_high());
    draw_score(35.0, Caption::new(high_score, None, Some(25.0), None));

    let score = format!("Score: {}", score.get_current_score());
    draw_score(60.0, Caption::new(score, None, Some(25.0), None));
}

fn draw_score(y:f32, caption : Caption) {
    draw_text(
        &caption.text,
        screen_width() - caption.get_dimensions().width - 10.0, 
        y,
        *&caption.font_size,
        *&caption.color
    );
}

fn put_text_in_center(y : Option<f32>, caption : Caption) {
    let y = y.unwrap_or(get_center_y());

    draw_text(
        &caption.text, 
        get_center_x() - caption.get_dimensions().width / 2.0, 
        y, 
        *&caption.font_size,
        *&caption.color
    );
}

fn get_center_x() -> f32 {
    screen_width() / 2.0   
}

fn get_center_y() -> f32 {
    screen_height() / 2.0   
}