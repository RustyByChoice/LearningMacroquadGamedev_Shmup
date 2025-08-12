use macroquad::prelude::*;
use crate::bullet_vector::BulletVector;
use crate::PlayerShip;
use crate::enemy_square::{GameEntity,EnemySquare};
use macroquad_particles::{self as particles, ColorCurve, Emitter, EmitterConfig};

pub struct EnemyVector {
    enemies: Vec<EnemySquare>,
    explosions: Vec<(Emitter, Vec2)>,
}

impl EnemyVector {
    const ENEMY_COLORS : [Color; 4] = [GRAY, BEIGE, PINK, RED];

    pub fn new() -> EnemyVector {
        let enemies = vec![];
        let explosions = vec![];

        return EnemyVector { enemies, explosions }
    }

    pub fn spawn_enemy(&mut self) {
        let size = rand::gen_range(16.0, 64.0);
        let color = rand::gen_range(0, Self::ENEMY_COLORS.len());

        self.enemies.push(EnemySquare::new(size, Self::ENEMY_COLORS[color]));
    }

    pub fn hide_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.shape.y < screen_height() + enemy.shape.size);
        self.enemies.retain(|enemy| !enemy.shape.collided);
        self.explosions.retain(|(explosion, _)| explosion.config.emitting);
    }

    pub fn move_enemies(&mut self, delta_time :f32) {
        for enemy in &mut self.enemies {
            enemy.shape.y += enemy.shape.speed * delta_time;
        }
    }

    pub fn draw_enemies(&mut self) {
        for enemy in &self.enemies {
            draw_rectangle(
                enemy.shape.x - enemy.shape.size / 2.0,
                enemy.shape.y - enemy.shape.size / 2.0,
                enemy.shape.size,
                enemy.shape.size,
                enemy.shape.color,
            );
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
                            amount: enemy.shape.size.round() as u32 * 2,
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
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve { start: RED, mid: ORANGE, end: RED },
        ..Default::default()
    }
}
