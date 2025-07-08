use macroquad::prelude::*;
use crate::shape::Shape;

#[derive(Clone)]
pub struct HeroCircle {
    pub shape: Shape,
}

impl HeroCircle {
    pub fn new(where_x : f32, where_y : f32, speed : f32) -> HeroCircle {
        return HeroCircle {
            shape: Shape {
                size: 32.0,
                speed: speed,
                x: where_x,
                y: where_y,
                color: YELLOW,
                collided: false,
            },
        };
    }

    pub fn as_circle(&self) -> Circle {
        Circle {
            x: self.shape.x,
            y: self.shape.y,
            r: self.shape.size,
        }
    }

    pub fn move_up(&mut self) {
        self.shape.y -= self.shape.speed;
        self.clamp_y();
    }

    pub fn move_down(&mut self) {
        self.shape.y += self.shape.speed;
        self.clamp_y();
    }

    pub fn move_left(&mut self) {
        self.shape.x -= self.shape.speed;
        self.clamp_x();
    }

    pub fn move_right(&mut self) {
        self.shape.x += self.shape.speed;
        self.clamp_x();
    }

    fn clamp_x(&mut self) {
        self.shape.x = clamp(self.shape.x, 0.0 + self.shape.size, screen_width() - self.shape.size);
    }

    fn clamp_y(&mut self) {
        self.shape.y = clamp(self.shape.y, 0.0 + self.shape.size, screen_height() - self.shape.size);
    }

    pub fn draw(&self) {
        draw_circle(self.shape.x, self.shape.y, self.shape.size, self.shape.color);
    }

    pub fn set_speed(&mut self, new_speed :f32) {
        self.shape.speed = new_speed;
    }
}

// TODO: read the documentation for EmitterConfig and try what happens if you change different values. Can you add a particle system that shoots particles out of the circle so it looks like a rocket exhaust?