use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .init_resource::<SpriteAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct SpriteAssets {
    pub bevy_one: Sprite,
    pub bevy_two: Sprite,
}

impl Default for SpriteAssets {
    fn default() -> Self {
        SpriteAssets {
            bevy_one: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                color: Color::rgb(0.0, 0.8, 0.0),
                ..default()
            },
            bevy_two: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                color: Color::rgb(0.8, 0.0, 0.0),
                ..default()
            },
        }
    }
}
