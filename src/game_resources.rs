use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum AssetKey {
    Ship,
    LaserBolts,
    // Add more texture keys as needed
}

struct TextureMap {
    key: AssetKey,
    file_name: String,
}

fn get_texture_map() -> Vec<TextureMap> {
    vec!
    [
        TextureMap { key: AssetKey::Ship, file_name: "ship.png".to_owned() },
        TextureMap { key: AssetKey::LaserBolts, file_name: "laser-bolts.png".to_owned() }
    ]
}

pub async fn load_textures() -> HashMap<AssetKey, Texture2D> {
    let map = get_texture_map();
    let mut hashes = HashMap::new();

    for texture_map in map {
        let error_message = format!("Couldn't load texture file {}", texture_map.file_name);

        let texture = load_texture(&texture_map.file_name).await.expect(&error_message);
        texture.set_filter(FilterMode::Nearest);

        hashes.insert(texture_map.key.clone(), texture);
    }

    hashes
}

pub trait TextureHashMapExtensions {
    fn take(&self, key: &AssetKey) -> &Texture2D;
}

impl TextureHashMapExtensions for HashMap<AssetKey, Texture2D> {
    fn take(&self, key: &AssetKey) -> &Texture2D {
        self.get(key).unwrap()
    }
}