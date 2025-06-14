use crate::shape::Shape;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Bullet {
    pub shape: Shape,
}

impl Bullet {
    pub fn new(start_x : f32, start_y : f32) -> Bullet {
        return Bullet {
            shape: Shape {
                x: start_x,
                y: start_y,
                speed: 50.0,
                size: 5.0,
                color: YELLOW,
                collided: false,
            },
        };
    }
    
    pub fn as_rect(&self) -> Rect {
        Rect {
            x: self.shape.x - self.shape.size / 2.0,
            y: self.shape.y - self.shape.size / 2.0,
            w: self.shape.size,
            h: self.shape.size,
        }
    }
}