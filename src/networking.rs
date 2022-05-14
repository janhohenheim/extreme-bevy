use crate::actions::{create_input_protocol, set_movement_actions, Actions};
use crate::config::FPS;
use crate::player::move_players;
use crate::GameState;
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, SessionType};
use ggrs::{Config, PlayerHandle, PlayerType, SessionBuilder};
#[cfg(not(target_arch = "wasm32"))]
mod native;
pub mod shared;
use protocol::GGRSConfig;
pub mod protocol;
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

        app.insert_resource(SessionType::P2PSession);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(wasm::WasmPlugin);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugin(native::NativePlugin);
    }
}
