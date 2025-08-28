use std::collections::HashMap;
use macroquad::prelude::*;
use crate::game_resources::AssetKey;

pub trait TextureHashMap {
    fn take(&self, key: &AssetKey) -> &Texture2D;
    fn take_enemies(&self) -> HashMap<AssetKey, Texture2D>;
}

impl TextureHashMap for HashMap<AssetKey, Texture2D> {
    fn take(&self, key: &AssetKey) -> &Texture2D {
        self.get(key).unwrap()
    }

    fn take_enemies(&self) -> HashMap<AssetKey, Texture2D> {
        let mut filtered = HashMap::new();

        filtered.insert(AssetKey::EnemySmall, self.take(&AssetKey::EnemySmall).to_owned());
        filtered.insert(AssetKey::EnemySmall, self.take(&AssetKey::EnemyMedium).to_owned());
        filtered.insert(AssetKey::EnemySmall, self.take(&AssetKey::EnemyBig).to_owned());

        filtered
    }
}