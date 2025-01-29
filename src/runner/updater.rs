use eyre::Result;
use gamepads::Button;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::KeyboardInput;
use super::Action;
use super::AppState;
use super::WorkingParams;
use hecs_wrapper::error::input_system;
use hecs_wrapper::prelude::*;

//  //  //  //  //  //  //  //
pub fn update(state: AppState, with_action: Action) -> Result<(AppState, Action)> {
    match (state, with_action) {
        (AppState::JustInited, Action::Tick) => {
            let mut world = Box::new(RaaWorld::new());
            world.repopulate();
            world.update_on_tick()?;

            let new_state = AppState::Working(WorkingParams {
                is_gamepad_connected: None,
                world,
            });
            return Ok((new_state, Action::Noop));
        }
        (AppState::Working(mut working), Action::Tick) => {
            working.world.update_on_tick()?;
            let new_state = AppState::Working(working);
            return Ok((new_state, Action::Noop));
        }
        (AppState::Working(mut working), Action::GameInput(input, restart)) => {
            if restart {
                return Ok((AppState::JustInited, Action::Noop));
            }
            if working.world.send_to_player(input)
                == Err(input_system::InputSystemError::NoPlayerToSend)
            {
                warn!("{}", input_system::InputSystemError::NoPlayerToSend);
            }
            let new_state = AppState::Working(working);
            return Ok((new_state, Action::Noop));
        }
        (AppState::Working(mut working), Action::ProcessMainGamepadInput(None)) => {
            working.is_gamepad_connected = Some(false);
            let new_state = AppState::Working(working);
            return Ok((new_state, Action::Noop));
        }
        (AppState::Working(mut working), Action::ProcessMainGamepadInput(Some(gamepad))) => {
            working.is_gamepad_connected = Some(true);
            let new_state = AppState::Working(working);
            let (input, restart) = translate_gamepad(&gamepad);
            return Ok((new_state, Action::GameInput(input, restart)));
        }
        (AppState::Working(working), Action::TranslateRawEvent(ev)) => {
            if let Some(true) = &working.is_gamepad_connected {
                let new_state = AppState::Working(working);
                return Ok((new_state, Action::Noop));
            }
            let new_state = AppState::Working(working);
            match translate_keyboard(ev) {
                None => {
                    return Ok((new_state, Action::Noop));
                }
                Some(kbd_inpt) => {
                    return Ok((new_state, Action::Keyboard(kbd_inpt)));
                }
            }
        }
        (AppState::Working(mut working), Action::Keyboard(kbd)) => match kbd {
            KeyboardInput::QuitRequest => {
                let new_state = AppState::Working(working);
                return Ok((new_state, Action::Quit));
            }
            KeyboardInput::GameRestart => {
                return Ok((AppState::JustInited, Action::Noop));
            }
            KeyboardInput::GameInput(game_input_command) => {
                if working.world.send_to_player(vec![game_input_command])
                    == Err(input_system::InputSystemError::NoPlayerToSend)
                {
                    warn!("{}", input_system::InputSystemError::NoPlayerToSend);
                }
                let new_state = AppState::Working(working);
                return Ok((new_state, Action::Noop));
            }
        },
        (_, Action::Quit) => {
            return Ok((AppState::Exiting, Action::Noop));
        }
        (state_out, _) => {
            return Ok((state_out, Action::Noop));
        }
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn translate_gamepad(gamepad: &gamepads::Gamepad) -> (Vec<GameInputCommand>, bool) {
    let mut cmds = Vec::new();
    for button in gamepad.all_just_pressed() {
        match button {
            Button::RightCenterCluster => {
                return (Vec::new(), true);
            }
            Button::DPadUp => cmds.push(GameInputCommand::OnceUp),
            Button::DPadDown => cmds.push(GameInputCommand::OnceDown),
            Button::DPadLeft => cmds.push(GameInputCommand::OnceLeft),
            Button::DPadRight => cmds.push(GameInputCommand::OnceRight),
            Button::ActionUp => cmds.push(GameInputCommand::TypeDigital('4')),
            Button::ActionRight => cmds.push(GameInputCommand::TypeDigital('3')),
            Button::ActionDown => cmds.push(GameInputCommand::TypeDigital('2')),
            Button::ActionLeft => cmds.push(GameInputCommand::TypeDigital('1')),
            _ => (),
        }
    }
    cmds.push(GameInputCommand::Accelerate(gamepad.left_stick()));

    (cmds, false)
}

//  //  //  //  //  //  //  //
use ratatui::crossterm::event as xEvent;

#[inline(always)]
fn translate_keyboard(event: xEvent::Event) -> Option<KeyboardInput> {
    if let xEvent::Event::Key(key) = event {
        match key.code {
            xEvent::KeyCode::Char('q') => {
                return Some(KeyboardInput::QuitRequest);
            }

            xEvent::KeyCode::Char('r') => {
                return Some(KeyboardInput::GameRestart);
            }

            xEvent::KeyCode::Char('k') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::OnceUp));
            }
            xEvent::KeyCode::Char('j') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::OnceDown));
            }
            xEvent::KeyCode::Char('h') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::OnceLeft));
            }
            xEvent::KeyCode::Char('l') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::OnceRight));
            }

            xEvent::KeyCode::Char('1') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::TypeDigital('1')));
            }
            xEvent::KeyCode::Char('2') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::TypeDigital('2')));
            }
            xEvent::KeyCode::Char('3') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::TypeDigital('3')));
            }
            xEvent::KeyCode::Char('4') => {
                return Some(KeyboardInput::GameInput(GameInputCommand::TypeDigital('4')));
            }

            _ => (),
        }
    }

    None
}
