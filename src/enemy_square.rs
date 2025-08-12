use macroquad::prelude::*;
use crate::shape::Shape;
use crate::hero_circle::HeroCircle;
use crate::bullet::Bullet;

pub enum GameEntity<'a> {
    Hero(HeroCircle<'a>),
    Projectile(Bullet),
}

pub struct EnemySquare {
    pub shape: Shape,
}

impl EnemySquare {
    pub fn new(size : f32, color : Color) -> EnemySquare {
        return EnemySquare {
            shape: Shape {
               size,
               speed: rand::gen_range(50.0, 150.0),
               x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
               y: -size,
               color: color,
               collided: false,
            },
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
}
