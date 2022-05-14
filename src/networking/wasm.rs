use super::shared::*;
use crate::GameState;
use bevy::{log, prelude::*, tasks::IoTaskPool};
use bevy_web_resizer::Plugin as WebResizerPlugin;
use matchbox_socket::WebRtcSocket;

pub struct WasmPlugin;
impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Using wasm networking plugin");
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wait_for_players));

        app.add_plugin(WebResizerPlugin);
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

    // create a GGRS P2P session
    let mut p2p_session = create_session_builder(num_players);

    let mut handles = Vec::new();
    for (i, player_type) in players.into_iter().enumerate() {
        if player_type == PlayerType::Local {
            handles.push(i);
        }
        p2p_session = p2p_session
            .add_player(player_type, i)
            .expect("Failed to add player");
    }

    // start the GGRS session
    let session = p2p_session
        .start_p2p_session(socket)
        .expect("Session could not be created.");
    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
}
