use crate::bullet::Bullet;
use macroquad::prelude::*;

pub struct BulletVector {
    pub bullets: Vec<Bullet>,
}

impl BulletVector {
    pub fn new() -> BulletVector {
        let bullets = vec![];

        return BulletVector { bullets };
    }

    pub fn move_bullets(&mut self, delta_time :f32) {
        for bullet in &mut self.bullets {
            bullet.shape.y -= bullet.shape.speed * delta_time;
        }
    }

    pub fn fire(&mut self, start_x : &f32, start_y : &f32) {
        self.bullets.push(Bullet::new(*start_x, *start_y));
    }

    pub fn hide_bullets(&mut self) {
        self.bullets.retain(|bullet| bullet.shape.y > 0.0 - bullet.shape.size / 2.0);
        self.bullets.retain(|bullet| !bullet.shape.collided);
    }

    pub fn draw_bullets(&mut self) {
        for bullet in &self.bullets {
            draw_circle_lines(
                bullet.shape.x,
                bullet.shape.y,
                bullet.shape.size / 2.0,
                1.0,
                RED,
            );
        }
    }

    pub fn clear(&mut self) {
        self.bullets.clear();
    }

}