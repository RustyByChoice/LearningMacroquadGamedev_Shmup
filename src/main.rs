mod shape;
mod bullet;
mod bullet_vector;
mod enemy_ship;
mod enemy_vector;
mod player_ship;
mod high_score;
mod caption;
mod starfield_shader;
mod macroquad_helpers;
mod resources;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui}; 
use macroquad::audio::{play_sound, play_sound_once, stop_sound, PlaySoundParams};
use macroquad::experimental::collections::storage;

use crate::bullet_vector::BulletVector;
use crate::player_ship::PlayerShip;
use crate::enemy_vector::EnemyVector;
use crate::high_score::HighScore;
use crate::caption::Caption;
use crate::starfield_shader::StarfieldShader;
use crate::macroquad_helpers::*;
use crate::resources::Resources;

const MOVEMENT_SPEED: f32 = 200.0;
const SHOT_FREQUENCY: f64 = 0.25;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver
}

#[macroquad::main("SHMUP'EM UP!")]
async fn main() -> Result<(), macroquad::Error> {
    rand::srand(miniquad::date::now() as u64);

    set_pc_assets_folder("assets");
    Resources::load().await?;
    let resources = storage::get::<Resources>();

    root_ui().push_skin(&resources.ui_skin);
    let window_size = vec2(370.0, 320.0);

    let mut starfield_shader : StarfieldShader = StarfieldShader::new(
        include_str!("shaders/starfield-shader.glsl"),
        include_str!("shaders/vertex-shader.glsl"),
    );

    let mut game_state = GameState::MainMenu;

    let mut enemy_vector: EnemyVector = EnemyVector::new(
        resources.enemy_small_texture.clone(),
        resources.enemy_medium_texture.clone(),
        resources.enemy_big_texture.clone(),
        resources.explosion_texture.clone()
    );
    let mut bullet_vector: BulletVector = BulletVector::new(&resources.bullet_texture); 
    let mut player_ship = PlayerShip::new(get_center_x(), get_center_y(), MOVEMENT_SPEED, &resources.ship_texture);

    let mut high_score = HighScore::new();
 
    loop {
        clear_background(BLACK);

        starfield_shader.render_starfield(screen_width(), screen_height());

        match game_state {
            GameState::MainMenu => {
                root_ui().window(
                    hash!(),
                    vec2(
                        get_center_x() - window_size.x / 2.0,
                        get_center_y() - window_size.x / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(80.0, -34.0), "Main Menu");
                        if ui.button(vec2(65.0, 25.0), "Play") {
                            enemy_vector.clear();
                            bullet_vector.clear();
                            high_score.clear();
                            player_ship = PlayerShip::new(get_center_x(), get_center_y(), MOVEMENT_SPEED, &resources.ship_texture);
                            game_state = GameState::Playing;
                        }
                        if ui.button(vec2(65.0, 125.0), "Quit") {
                            std::process::exit(0);
                        }
                    }
                );

                // TODO 1: make the ui navigable by keyboard
                // let title = Caption::new(
                //     "SHMUP'EM UP!".to_string(),
                //     None,
                //     Some(100.0),
                //     None
                // );

                // put_text_in_center(Some(get_center_y() - title.get_dimensions().height), title);
                // put_text_in_center(None, Caption::default("Press space"));
            }
            GameState::Playing => {
                // time that passed since the last frame
                let delta_time = get_frame_time();
                let play_music = false;

                if play_music {
                    play_sound(
                        &resources.theme_music,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                }

                player_ship.set_speed(MOVEMENT_SPEED * delta_time);

                if rand::gen_range(0, 99) >= 95 {
                    enemy_vector.spawn_enemy();
                }

                player_ship.set_idle();
                if is_key_down(KeyCode::Right) {
                    player_ship.move_right();
                    starfield_shader.direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    player_ship.move_left();
                    starfield_shader.direction_modifier -= 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Down) {
                    player_ship.move_down();
                }
                if is_key_down(KeyCode::Up) {
                    player_ship.move_up();
                }
                if is_key_pressed(KeyCode::Space) {
                    let shots_fired = get_time();

                    if shots_fired > bullet_vector.last_time_fired + SHOT_FREQUENCY {
                        bullet_vector.fire(&player_ship.shape.x, &(player_ship.shape.y - 24.0));
                        bullet_vector.last_time_fired = shots_fired;
                    }
                    play_sound_once(&resources.sound_laser);
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

                player_ship.update_sprite();

                // COLLISION DETECTION
                if enemy_vector.collides_with(player_ship.clone()) {
                    high_score.save_high_score();
                    game_state = GameState::GameOver;
                }

                if enemy_vector.collides_with_bullets(&mut bullet_vector) {
                    high_score.add();
                    play_sound_once(&resources.sound_explosion);
                }

                // DRAW
                enemy_vector.draw_enemies();
                bullet_vector.draw_bullets();
                player_ship.draw();
                draw_high_score(&high_score);                
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                stop_sound(&resources.theme_music);
                put_text_in_center(None, Caption::default("Paused"));
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                stop_sound(&resources.theme_music);
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
