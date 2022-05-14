use crate::actions::Actions;
use crate::config::FPS;
use crate::loading::{SpriteAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_players)
                .with_system(spawn_camera),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    // 1 unit ≙ 50 px
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands
        .spawn_bundle(camera_bundle)
        .insert(Name::new("2D Camera"));
}

fn spawn_players(mut commands: Commands, textures: Res<TextureAssets>, sprites: Res<SpriteAssets>) {
    spawn_player(
        &mut commands,
        &textures,
        &sprites.bevy_one,
        Vec3::new(-2.0, 0.0, 0.0),
    );
    spawn_player(
        &mut commands,
        &textures,
        &sprites.bevy_two,
        Vec3::new(2.0, 0.0, 0.0),
    );
}

fn spawn_player(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    sprite: &Sprite,
    translation: Vec3,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(translation),
            sprite: sprite.clone(),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player);
}

pub fn move_player(actions: Res<Actions>, mut player_query: Query<&mut Transform, With<Player>>) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 15. / FPS as f32;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed,
        actions.player_movement.unwrap().y * speed,
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
