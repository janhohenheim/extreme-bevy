use crate::networking::{InputFlags, InputProtocol, LocalHandles};
use crate::GameState;
use bevy::log;
use bevy::prelude::*;
use ggrs::{InputStatus, PlayerHandle};
pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
        //.add_system_set(
        //    SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        //)
        ;
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    inputs: Res<Vec<(InputProtocol, InputStatus)>>,
) {
    let input: InputFlags = inputs[0].0.try_into().unwrap();

    if input.is_empty() {
        actions.player_movement = None;
        return;
    }

    let mut player_movement = Vec2::ZERO;
    if input.contains(InputFlags::LEFT) {
        player_movement.x -= 1.;
    }
    if input.contains(InputFlags::RIGHT) {
        player_movement.x += 1.;
    }
    if input.contains(InputFlags::UP) {
        player_movement.y += 1.;
    }
    if input.contains(InputFlags::DOWN) {
        player_movement.y -= 1.;
    }

    if player_movement == Vec2::ZERO {
        return;
    }

    player_movement = player_movement.normalize();
    actions.player_movement = Some(player_movement);
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Fire,
}

macro_rules! generate_bindings {
    ( $( $game_control:pat => $key_codes:expr ),+ ) => {

            impl GameControl {
                #[allow(dead_code)]
                fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_just_released($key_codes),
                        )+
                    }
                }

                #[allow(dead_code)]
                fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_just_pressed($key_codes),
                        )+
                    }
                }

                fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_pressed($key_codes),
                        )+
                    }
                }
            }

    };
}

generate_bindings! {
    GameControl::Up => [KeyCode::W, KeyCode::Up,],
    GameControl::Down => [KeyCode::S, KeyCode::Down,],
    GameControl::Left => [KeyCode::A, KeyCode::Left,],
    GameControl::Right => [KeyCode::D, KeyCode::Right,],
    GameControl::Fire => [KeyCode::Space, KeyCode::Return]
}

pub fn create_input_protocol(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
    _local_handles: Res<LocalHandles>,
) -> InputProtocol {
    let mut input = InputFlags::empty();

    if GameControl::Up.pressed(&keyboard_input) {
        input |= InputFlags::UP;
    }
    if GameControl::Down.pressed(&keyboard_input) {
        input |= InputFlags::DOWN;
    }
    if GameControl::Left.pressed(&keyboard_input) {
        input |= InputFlags::LEFT;
    }
    if GameControl::Right.pressed(&keyboard_input) {
        input |= InputFlags::RIGHT;
    }
    if GameControl::Fire.pressed(&keyboard_input) {
        input |= InputFlags::FIRE;
    }

    input.into()
}
