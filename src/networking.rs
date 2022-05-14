use crate::actions::create_input_protocol;
use crate::GameState;
use bevy::{log, prelude::*, tasks::IoTaskPool};
use bevy_ggrs::{GGRSPlugin, SessionType};
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, PlayerHandle, SessionBuilder};
use matchbox_socket::WebRtcSocket;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            .with_input_system(create_input_protocol)
            .build(app);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wait_for_players));
    }
}

fn start_matchbox_socket(mut commands: Commands, task_pool: Res<IoTaskPool>) {
    let room_url = "ws://127.0.0.1:3536/next_2";
    log::info!("Connecting to matchbox server: {}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    task_pool.spawn(message_loop).detach();
    commands.insert_resource(Some(socket));
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

fn wait_for_players(mut commands: Commands, mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();
    if socket.is_none() {
        // If there is no socket we've already started the game
        return;
    }
    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    log::info!("All players have joined, starting game");

    // consume the socket (currently required because GGRS takes ownership of its socket)
    let socket = socket.take().unwrap();

    let max_predictions = 12;

    // create a GGRS P2P session
    let mut p2p_session: SessionBuilder<GGRSConfig> = SessionBuilder::new()
        .with_num_players(num_players)
        .with_max_prediction_window(max_predictions)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        p2p_session = p2p_session
            .add_player(player, i)
            .expect("Failed to add player");
    }

    // start the GGRS session
    let session = p2p_session
        .start_p2p_session(socket)
        .expect("Session could not be created.");
    commands.insert_resource(session);
    commands.insert_resource(SessionType::P2PSession);
}
