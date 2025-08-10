use std::collections::HashMap;
use macroquad::prelude::*;
use crate::game_resources::AssetKey;

pub trait TextureHashMap {
    fn take(&self, key: &AssetKey) -> &Texture2D;
}

impl TextureHashMap for HashMap<AssetKey, Texture2D> {
    fn take(&self, key: &AssetKey) -> &Texture2D {
        self.get(key).unwrap()
    }
}