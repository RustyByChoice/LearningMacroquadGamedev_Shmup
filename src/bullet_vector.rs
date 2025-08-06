use macroquad::experimental::animation::{AnimatedSprite, Animation};
use crate::bullet::Bullet;
use macroquad::prelude::*;

pub struct BulletVector {
    pub bullets: Vec<Bullet>,
    pub last_time_fired : f64,
    pub bullet_texture: Texture2D,
    pub bullet_sprite: AnimatedSprite,
}

impl BulletVector {
    pub fn new(texture_bullet: Texture2D) -> BulletVector {
        let bullets = vec![];

        let bullet_sprite = AnimatedSprite::new(
            16, 16,
            &[
                Animation {
                    name: "bullet".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12
                },
                Animation {
                    name: "bolt".to_string(),
                    row: 1,
                    frames: 2,
                    fps: 12
                }
            ],
            true
        );

        BulletVector {
            bullets: bullets,
            last_time_fired: 0.,
            bullet_texture: texture_bullet,
            bullet_sprite: bullet_sprite
        }
    }

    pub fn move_bullets(&mut self, delta_time :f32) {
        for bullet in &mut self.bullets {
            bullet.shape.y -= bullet.shape.speed * delta_time;
        }
        self.bullet_sprite.update();
    }

    pub fn fire(&mut self, start_x : &f32, start_y : &f32) {
        self.bullets.push(Bullet::new(*start_x, *start_y));
    }

    pub fn hide_bullets(&mut self) {
        self.bullets.retain(|bullet| bullet.shape.y > 0.0 - bullet.shape.size / 2.0);
        self.bullets.retain(|bullet| !bullet.shape.collided);
    }

    pub fn draw_bullets(&mut self) {
        let bullet_frame = self.bullet_sprite.frame();
        for bullet in &self.bullets {
            draw_texture_ex(
                &self.bullet_texture,
                bullet.shape.x - bullet.shape.size / 2.0,
                bullet.shape.y - bullet.shape.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bullet.shape.size, bullet.shape.size)),
                    source: Some(bullet_frame.source_rect),
                    ..Default::default()
                }
            );
            // draw_circle_lines(
            //     bullet.shape.x,
            //     bullet.shape.y,
            //     bullet.shape.size / 2.0,
            //     1.0,
            //     RED,
            // );
        }
    }

    pub fn clear(&mut self) {
        self.bullets.clear();
    }

}