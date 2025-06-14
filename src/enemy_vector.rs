use macroquad::prelude::*;
use crate::HeroCircle;
use crate::bullet::Bullet;
use crate::enemy_square::{GameEntity,EnemySquare};

pub struct EnemyVector {
    enemies: Vec<EnemySquare>,
}

impl EnemyVector {
    pub fn new() -> EnemyVector {
        let enemies = vec![];

        return EnemyVector { enemies };
    }

    pub fn spawn_enemy(&mut self, size :f32, color :Color) {
        self.enemies.push(EnemySquare::new(size, color));
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
        // self.enemies.iter().any(|e| circle.collides_with(e.rect()))
        self.enemies.iter_mut().any(|e| e.collides_with(GameEntity::Hero(circle.clone())))
    }

    pub fn collides_with_bullets(&mut self, bullets :&mut Vec<Bullet>) {
        for bullet in bullets.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if enemy.collides_with(GameEntity::Projectile(bullet.clone())) {
                    bullet.shape.collided = true;
                    enemy.shape.collided = true;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.enemies.clear();
    }
}
