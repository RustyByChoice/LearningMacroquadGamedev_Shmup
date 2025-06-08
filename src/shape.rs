use macroquad::prelude::{Color,Rect,*};

pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
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

pub struct HeroCircle {
    shape: Shape,
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
        self.circle().overlaps_rect(&other.shape.rect())
    }

    fn circle(&self) -> Circle {
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
