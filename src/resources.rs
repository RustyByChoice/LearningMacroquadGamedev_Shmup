use macroquad::prelude::{FilterMode, Texture2D, build_textures_atlas, load_image, load_file, RectOffset, WHITE, load_texture};
use macroquad::audio::{Sound, load_sound};
use macroquad::ui::{Skin, root_ui};

pub struct Resources {
    pub ship_texture: Texture2D,
    pub bullet_texture: Texture2D,
    pub explosion_texture: Texture2D,
    pub enemy_small_texture: Texture2D,
    pub enemy_medium_texture: Texture2D,
    pub enemy_big_texture: Texture2D,
    pub theme_music: Sound,
    pub sound_explosion: Sound,
    pub sound_laser: Sound,
    pub ui_skin: Skin,
}

impl Resources {
    pub async fn new() -> Result<Resources, macroquad::Error> {
        let ship_texture = Self::load_texture("ship.png").await;
        let bullet_texture = Self::load_texture("laser-bolts.png").await;
        let explosion_texture = Self::load_texture("explosion.png").await;
        let enemy_small_texture = Self::load_texture("enemy-small.png").await;
        let enemy_medium_texture = Self::load_texture("enemy-medium.png").await;
        let enemy_big_texture = Self::load_texture("enemy-big.png").await;
        // ensure that draw_texture calls will use atlas and not separate textures
        build_textures_atlas();

        let theme_music = load_sound("8bit-spaceshooter.ogg").await?;
        let sound_explosion = load_sound("explosion.wav").await?;
        let sound_laser = load_sound("laser.wav").await?;

        let ui_skin = Self::prepare_ui_skin().await;

        Ok(Resources {
            ship_texture,
            bullet_texture,
            explosion_texture,
            enemy_small_texture,
            enemy_medium_texture,
            enemy_big_texture,
            theme_music,
            sound_explosion,
            sound_laser,
            ui_skin: ui_skin?,
        })
    }

    pub async fn load_texture(texture_name:&str) -> Texture2D {
        let texture = load_texture(texture_name)
            .await.expect(&format!("Couldn't load texture file {}", texture_name));
        texture.set_filter(FilterMode::Nearest);

        texture
    }

    async fn prepare_ui_skin() -> Result<Skin, macroquad::Error> {
        let window_background = load_image("window_background.png").await?;
        let button_background = load_image("button_background.png").await?;
        let button_clicked_background = load_image("button_clicked_background.png").await?;
        let font = load_file("atari_games.ttf").await?;

        let window_style = root_ui()
            .style_builder()
            .background(window_background)
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(button_background)
            .background_clicked(button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build();

        let label_style = root_ui()
            .style_builder()
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build();

        Ok(Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        })
    }
}