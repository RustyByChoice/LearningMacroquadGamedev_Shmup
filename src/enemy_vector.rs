use macroquad::prelude::*;
use crate::bullet_vector::BulletVector;
use crate::HeroCircle;
use crate::enemy_square::{GameEntity,EnemySquare};

pub struct EnemyVector {
    enemies: Vec<EnemySquare>,
}

impl EnemyVector {
    const ENEMY_COLORS : [Color; 4] = [GRAY, BEIGE, PINK, RED];

    pub fn new() -> EnemyVector {
        let enemies = vec![];

        return EnemyVector { enemies };
    }

    pub fn spawn_enemy(&mut self) {
        let size = rand::gen_range(16.0, 64.0);
        let color = rand::gen_range(0, Self::ENEMY_COLORS.len());

        self.enemies.push(EnemySquare::new(size, Self::ENEMY_COLORS[color]));
    }

    pub fn hide_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.shape.y < screen_height() + enemy.shape.size);
        self.enemies.retain(|enemy| !enemy.shape.collided);
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
    }

    pub fn collides_with(&mut self, circle : HeroCircle) -> bool {
        self.enemies
            .iter_mut()
            .any(|e| e.collides_with(GameEntity::Hero(circle.clone())))
    }

    pub fn collides_with_bullets(&mut self, bullets : &mut BulletVector) -> bool {
        for bullet in bullets.bullets.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if enemy.collides_with(GameEntity::Projectile(bullet.clone())) {
                    bullet.shape.collided = true;
                    enemy.shape.collided = true;
                    return true;
                }
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.enemies.clear();
    }
}
