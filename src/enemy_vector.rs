use crate::bullet_vector::BulletVector;
use crate::PlayerShip;
use crate::enemy_ship::{GameEntity,EnemyShip};

use macroquad::prelude::*;
use macroquad::prelude::animation::Animation;
use macroquad::prelude::animation::AnimatedSprite;
use macroquad_particles::{self as particles, AtlasConfig, Emitter, EmitterConfig};

pub struct EnemyVector {
    enemies: Vec<EnemyShip>,
    explosions: Vec<(Emitter, Vec2)>,
    small_enemy_texture: Texture2D,
    medium_enemy_texture: Texture2D,
    big_enemy_texture: Texture2D,
    explosion_texture: Texture2D,
}

impl EnemyVector {
    pub fn new(
        small_enemy_texture: Texture2D,
        medium_enemy_texture: Texture2D,
        big_enemy_texture: Texture2D, 
        explosion_texture: Texture2D
    ) -> EnemyVector {
        EnemyVector { 
            enemies: vec![], 
            explosions: vec![], 
            small_enemy_texture: small_enemy_texture, 
            medium_enemy_texture: medium_enemy_texture, 
            big_enemy_texture: big_enemy_texture, 
            explosion_texture: explosion_texture
        }
    }

    // TODO 1: spawn many types of sizes of enemies
    pub fn spawn_enemy(&mut self) {
        // let sizes = vec!(AssetKey::EnemySmall, AssetKey::EnemyMedium, AssetKey::EnemyBig);
        // let size_texture = rand::gen_range(0, 2);
        // let random_size = &sizes[size_texture];
        let picked_size = self.small_enemy_texture.clone();

        // let size = rand::gen_range(16.0, 64.0);

        let enemy_sprite_small : AnimatedSprite = AnimatedSprite::new(
            17,
            16,
            &[Animation {
                name: "enemy_small".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true
        );

        self.enemies.push(EnemyShip::new(17.0, picked_size, enemy_sprite_small));
    }

    pub fn hide_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.shape.y < screen_height() + enemy.shape.size);
        self.enemies.retain(|enemy| !enemy.shape.collided);
        self.explosions.retain(|(explosion, _)| explosion.config.emitting);
    }

    pub fn move_enemies(&mut self, delta_time :f32) {
        for enemy in &mut self.enemies {
            enemy.shape.y += enemy.shape.speed * delta_time;
            enemy.sprite.update();
        }
    }

    pub fn draw_enemies(&mut self) {
        for enemy in &self.enemies {
            enemy.draw();
        }

        for (explosion, coords) in self.explosions.iter_mut() {
            explosion.draw(*coords);
        }
    }

    pub fn collides_with(&mut self, player : PlayerShip) -> bool {
        self.enemies
            .iter_mut()
            .any(|e| e.collides_with(GameEntity::Hero(player.clone())))
    }

    pub fn collides_with_bullets(&mut self, bullets : &mut BulletVector) -> bool {
        for bullet in bullets.bullets.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if enemy.collides_with(GameEntity::Projectile(bullet.clone())) {
                    bullet.shape.collided = true;
                    enemy.shape.collided = true;

                    self.explosions.push((
                        Emitter::new(EmitterConfig { 
                            amount: enemy.shape.size.round() as u32 * 4,
                            texture: Some(self.explosion_texture.clone()),
                            ..particle_explosion()
                        }),
                        vec2(enemy.shape.x, enemy.shape.y),
                    ));

                    return true;
                }
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.enemies.clear();
        self.explosions.clear();
    }
}

fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 400.0,
        initial_velocity_randomness: 0.8,
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5, 1, 0..)),
        ..Default::default()
    }
}
