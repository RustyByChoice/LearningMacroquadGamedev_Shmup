use macroquad::prelude::{Color,Rect,*};

pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

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
            },
        };
    }

    pub fn collides_with(&self, other: &EnemySquare) -> bool {
        self.rect().overlaps(&other.shape.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.shape.x - self.shape.size / 2.0,
            y: self.shape.y - self.shape.size / 2.0,
            w: self.shape.size,
            h: self.shape.size,
        }
    }
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
            },
        };
    }
}

impl Shape {
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
