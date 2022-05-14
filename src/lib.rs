mod actions;
mod audio;
mod dev;
mod loading;
mod menu;
mod networking;
mod player;
mod config;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::dev::DevPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::networking::NetworkingPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(NetworkingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(DevPlugin);
    }
}
