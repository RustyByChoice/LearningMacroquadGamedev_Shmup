use macroquad::prelude::*;
use crate::Shape;

pub struct EnemyVector {
    enemies: Vec<Shape>,
}

impl EnemyVector {
    pub fn new() -> EnemyVector {
        let enemies = vec![];

        return EnemyVector { enemies };
    }

    pub fn spawn_enemies(&mut self) {
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            self.enemies.push(Shape {
               size,
               speed: rand::gen_range(50.0, 150.0),
               x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
               y: -size,
            });
        }
    }

    pub fn hide_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.y < screen_height() + enemy.size);
    }

    pub fn move_enemies(&mut self, delta_time :f32) {
        for enemy in &mut self.enemies {
            enemy.y += enemy.speed * delta_time;
        }
    }

    pub fn draw_enemies(&mut self) {
        for enemy in &self.enemies {
            draw_rectangle(
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                enemy.size,
                enemy.size,
                GREEN,
            );
        }
    }
}
