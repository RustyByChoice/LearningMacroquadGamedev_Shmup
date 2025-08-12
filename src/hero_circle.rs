use macroquad::prelude::*;
use crate::shape::Shape;
use macroquad::experimental::animation::{AnimatedSprite, Animation};

#[derive(Clone)]
pub struct HeroCircle<'a> {
    pub shape: Shape,
    pub ship_sprite: AnimatedSprite,
    pub ship_texture: &'a Texture2D,
}

impl HeroCircle<'_> {
    pub fn new(where_x : f32, where_y : f32, speed : f32, texture_ship: &Texture2D) -> HeroCircle {

        let ship_sprite = AnimatedSprite::new(
            16, 24,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12
                },
                Animation {
                    name: "left".to_string(),
                    row: 2,
                    frames: 2,
                    fps: 12
                },
                Animation {
                    name: "right".to_string(),
                    row: 4,
                    frames: 2,
                    fps: 12
                },
            ],
            true,
        );

        return HeroCircle {
            shape: Shape {
                size: 32.0,
                speed: speed,
                x: where_x,
                y: where_y,
                color: YELLOW,
                collided: false,
            },
            ship_sprite: ship_sprite,
            ship_texture: texture_ship,
        };
    }

    pub fn as_circle(&self) -> Circle {
        Circle {
            x: self.shape.x,
            y: self.shape.y,
            r: self.shape.size,
        }
    }

    pub fn set_idle(&mut self) {
        self.ship_sprite.set_animation(0);
    }

    pub fn update_sprite(&mut self) {
        self.ship_sprite.update();
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
        self.ship_sprite.set_animation(1);
    }

    pub fn move_right(&mut self) {
        self.shape.x += self.shape.speed;
        self.clamp_x();
        self.ship_sprite.set_animation(2);
    }

    fn clamp_x(&mut self) {
        self.shape.x = clamp(self.shape.x, 0.0 + self.shape.size, screen_width() - self.shape.size);
    }

    fn clamp_y(&mut self) {
        self.shape.y = clamp(self.shape.y, 0.0 + self.shape.size, screen_height() - self.shape.size);
    }

    pub fn draw(&self) {
        let ship_frame = self.ship_sprite.frame();
        draw_texture_ex(
            &self.ship_texture,
            self.shape.x - ship_frame.dest_size.x,
            self.shape.y - ship_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ship_frame.dest_size * 2.0),
                source: Some(ship_frame.source_rect),
                ..Default::default()
            }
        );
    }

    pub fn set_speed(&mut self, new_speed :f32) {
        self.shape.speed = new_speed;
    }
}

// TODO: read the documentation for EmitterConfig and try what happens if you change different values. Can you add a particle system that shoots particles out of the circle so it looks like a rocket exhaust?