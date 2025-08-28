use macroquad::prelude::*;
use std::collections::HashMap;
use strum_macros::{EnumIter, EnumCount};

// TODO: Can enum variants pass Texture2D as objects in them? What is the gain?

#[derive(Hash, Eq, PartialEq, Clone, Debug, EnumIter, EnumCount)]
pub enum AssetKey {
    Ship,
    LaserBolts,
    Explosion,
    EnemySmall,
    EnemyMedium,
    EnemyBig
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
        TextureMap { key: AssetKey::LaserBolts, file_name: "laser-bolts.png".to_owned() },
        TextureMap { key: AssetKey::Explosion, file_name: "explosion.png".to_owned() },
        TextureMap { key: AssetKey::EnemySmall, file_name: "enemy-small.png".to_owned() },
        TextureMap { key: AssetKey::EnemyMedium, file_name: "enemy-medium.png".to_owned() },
        TextureMap { key: AssetKey::EnemyBig, file_name: "enemy-big.png".to_owned() },
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

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn ensure_that_all_asset_keys_have_textures_loaded() {
        use std::collections::HashSet;

        let all_variants: Vec<AssetKey> = AssetKey::iter().collect();
        let texture_maps = get_texture_map();

        let texture_keys: HashSet<&AssetKey> = texture_maps.iter().map(|t| &t.key).collect();

        for variant in &all_variants {
            assert!(
                texture_keys.contains(variant),
                "Missing texture map entry for variant: {:?}",
                variant
            );
        }

        // check if there are extra entries in the map that don't correspond to a known variant.
        // This ensures a 1-to-1 mapping.
        assert_eq!(
            all_variants.len(),
            texture_keys.len(),
            "The number of entries in get_texture_map() does not match the number of enum variants."
        );
    }
}
