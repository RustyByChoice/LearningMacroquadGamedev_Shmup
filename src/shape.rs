use macroquad::prelude::Color;

#[derive(Clone)]
pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub collided: bool,
}
