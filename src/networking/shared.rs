use crate::config::FPS;
use bitflags::bitflags;
use ggrs::{Config, PlayerHandle, SessionBuilder};

use super::protocol::InputProtocol;

/// You need to define a config struct to bundle all the generics of GGRS. You can safely ignore `State` and leave it as u8 for all GGRS functionality.
/// Source: https://github.com/gschup/bevy_ggrs/blob/7d3def38720161610313c7031d6f1cb249098b43/examples/box_game/box_game.rs#L27
#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = InputProtocol;
    type State = u8;
    type Address = String;
}

pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}

pub fn create_session_builder(num_players: usize) -> SessionBuilder<GGRSConfig> {
    SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(20)
        .with_fps(FPS)
        .expect("Invalid FPS")
        .with_input_delay(6)
}
