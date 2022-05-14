use std::net::SocketAddr;

use crate::GameState;

use super::shared::*;
use super::GGRSConfig;
use bevy::{log, prelude::*};
use bevy_ggrs::RollbackIdProvider;
use clap::Parser;
use ggrs::{P2PSession, PlayerType, UdpNonBlockingSocket};

pub struct NativePlugin;
impl Plugin for NativePlugin {
    fn build(&self, app: &mut App) {
        log::info!("Using native networking plugin");
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_session))
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    local_port: u16,
    #[clap(short, long)]
    players: Vec<String>,
}

fn start_session(mut commands: Commands) {
    let args = Args::parse();
    let num_players = args.players.len();
    assert!(num_players == 2);

    // create a GGRS session
    let mut p2p_session = create_session_builder(num_players);
    let mut handles = Vec::new();
    // add players
    for (i, player_addr) in args.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            p2p_session = p2p_session
                .add_player(PlayerType::Local, i)
                .expect("Failed to add local player");
            handles.push(i);
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse().unwrap();
            p2p_session = p2p_session
                .add_player(PlayerType::Remote(remote_addr), i)
                .expect("Failed to add remote player");
        }
    }

    // start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port(args.local_port).unwrap();
    let session = p2p_session.start_p2p_session(socket).unwrap();

    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
}
