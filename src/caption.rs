use macroquad::prelude::*;
use core::option::Option;

const FONT_SIZE : f32 = 50.0;
const FONT_SCALE : f32 = 1.0;
const FONT_COLOR : Color = WHITE;

pub struct Caption {
    pub text : String,
    pub color : Color,
    pub font_size : f32,
    pub font_scale : f32,
}

impl Caption {
    pub fn new(text : String, color : Option<Color>, font_size : Option<f32>, font_scale : Option<f32>) -> Caption {
        let color = color.unwrap_or(FONT_COLOR);
        let font_size = font_size.unwrap_or(FONT_SIZE);
        let font_scale = font_scale.unwrap_or(FONT_SCALE);

        Caption {
            text,
            color,
            font_size,
            font_scale,
        }
    }

    pub fn get_dimensions(&self) -> TextDimensions {
        measure_text(
            &self.text,
            None, 
            self.font_size as u16, 
            self.font_scale)
    }

    pub fn default(text : &str) -> Caption {
        Caption::new(text.to_string(),None,None,None)
    }
}