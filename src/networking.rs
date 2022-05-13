use crate::GameState;
use bevy::{log, prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcNonBlockingSocket;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wait_for_players));
    }
}

fn start_matchbox_socket(mut commands: Commands, task_pool: Res<IoTaskPool>) {
    let room_url = "ws://127.0.0.1:3536/next_2";
    log::info!("Connecting to matchbox server: {}", room_url);
    let (socket, message_loop) = WebRtcNonBlockingSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    task_pool.spawn(message_loop).detach();
    commands.insert_resource(Some(socket));
}

fn wait_for_players(mut socket: ResMut<Option<WebRtcNonBlockingSocket>>) {
    if socket.is_none() {
        // If there is no socket we've already started the game
        return;
    }
    let socket = socket.as_mut().as_mut().unwrap();
    // Check for new connections
    socket.accept_new_connections();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    log::info!("All players have joined, starting game");
}
