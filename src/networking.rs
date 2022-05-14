use crate::actions::{create_input_protocol, set_movement_actions, Actions};
use crate::config::FPS;
use crate::player::move_players;
use crate::GameState;
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, SessionType};
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, PlayerHandle, PlayerType, SessionBuilder};
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub struct NetworkingPlugin;
const ROLLBACK_SYSTEMS: &str = "rollback_systems";

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
enum Systems {
    Input,
    Move,
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            .with_input_system(create_input_protocol)
            .with_update_frequency(FPS)
            .with_rollback_schedule(
                Schedule::default().with_stage(
                    ROLLBACK_SYSTEMS,
                    SystemStage::parallel()
                        .with_system_set(State::<GameState>::get_driver())
                        .with_system_set(
                            SystemSet::on_update(GameState::Playing)
                                .with_system(set_movement_actions.label(Systems::Input))
                                .with_system(
                                    move_players.label(Systems::Move).after(Systems::Input),
                                ),
                        ),
                ),
            )
            .register_rollback_type::<Transform>()
            .register_rollback_type::<Actions>()
            .build(app);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(wasm::WasmPlugin);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugin(native::NativePlugin);
    }
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct InputProtocol {
    /// This is the number of bytes one peer’s input is.
    /// In our case, the input consists of four direction buttons, and eventually the fire button as well.
    /// This means it fits easily within a single byte:
    pub input: u8,
}

impl InputProtocol {
    pub fn new(input: InputFlags) -> Self {
        InputProtocol {
            input: input.bits(),
        }
    }
}

bitflags! {
    pub struct InputFlags: u8 {
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
        const FIRE = 1 << 4;
    }
}

impl From<InputFlags> for InputProtocol {
    fn from(input: InputFlags) -> Self {
        Self::new(input)
    }
}

impl TryFrom<InputProtocol> for InputFlags {
    type Error = String;
    fn try_from(protocol: InputProtocol) -> Result<Self, Self::Error> {
        Self::from_bits(protocol.input).ok_or_else(|| {
            format!(
                "Failed to read protocol bits as valid inputs. Received {}",
                protocol.input
            )
        })
    }
}
