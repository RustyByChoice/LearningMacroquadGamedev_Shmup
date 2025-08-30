use macroquad::prelude::*;
use macroquad::prelude::animation::AnimatedSprite;
use crate::shape::Shape;
use crate::player_ship::PlayerShip;
use crate::bullet::Bullet;

pub enum GameEntity<'a> {
    Hero(PlayerShip<'a>),
    Projectile(Bullet),
}

pub struct EnemyShip {
    pub shape: Shape,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
}

impl EnemyShip {
    pub fn new(size : f32, texture : Texture2D, sprite: AnimatedSprite) -> EnemyShip {
        return EnemyShip {
            shape: Shape {
               size,
               speed: rand::gen_range(50.0, 150.0),
               x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
               y: -size,
               color: WHITE,
               collided: false,
            },
            texture: texture,
            sprite: sprite
        };
    }

    fn as_rect(&self) -> Rect {
        Rect {
            x: self.shape.x - self.shape.size / 2.0,
            y: self.shape.y - self.shape.size / 2.0,
            w: self.shape.size,
            h: self.shape.size,
        }
    }

    pub fn collides_with(&mut self, object : GameEntity) -> bool {
        match object {
            GameEntity::Hero(hero) => {
                return hero.as_circle().overlaps_rect(&self.as_rect());
            },
            GameEntity::Projectile(bullet) => {
                return bullet.as_rect().overlaps(&self.as_rect());
            },
        }
    }

    pub fn draw(&self) {
        let enemy_frame = &self.sprite.frame();

        draw_texture_ex(
            &self.texture,
            self.shape.x - self.shape.size / 2.0,
            self.shape.y - self.shape.size / 2.0,
            self.shape.color,
            DrawTextureParams {
                dest_size: Some(vec2(self.shape.size, self.shape.size)),
                source: Some(enemy_frame.source_rect),
                ..Default::default()
            },
        );
    }
}
