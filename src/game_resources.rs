use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(Hash, Eq, PartialEq)]
pub enum AssetKey {
    Ship,
    Laser_Bolts,
    // Add more texture keys as needed
}

pub struct TextureManager {
    textures: HashMap<AssetKey, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_textures() -> 
    pub async fn load_texture(&mut self, key: AssetKey, path: &str) -> Result<(), String> {
        let texture = load_texture(path)
            .await
            .map_err(|e| format!("Failed to load texture: {}", e))?;
        
        texture.set_filter(FilterMode::Nearest);
        self.textures.insert(key, texture);
        Ok(())
    }

    pub fn get_texture(&self, key: &AssetKey) -> Option<&Texture2D> {
        self.textures.get(key)
    }
}