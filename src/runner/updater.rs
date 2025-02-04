use eyre::Result;
use gamepads::Button;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::KeyboardInput;
use super::Action;
use super::Action::*;
use super::AppState;
use super::AppState::*;
use super::WorkingState;
use hecs_wrapper::error::input_system;
use hecs_wrapper::prelude::*;

//  //  //  //  //  //  //  //
pub fn update(state: AppState, with_action: Action) -> Result<(AppState, Action)> {
    match (state, with_action) {
        (JustInited, Tick) => {
            let mut world = Box::new(RaaWorld::new());
            world.repopulate();
            world.update_on_tick(super::DELTA_TIME)?;

            let new_state = Working(WorkingState {
                is_gamepad_connected: None,
                world,
            });
            return Ok((new_state, Noop));
        }
        (Working(mut work_state), Tick) => {
            work_state.world.update_on_tick(super::DELTA_TIME)?;
            let new_state = Working(work_state);
            return Ok((new_state, Noop));
        }
        (Working(mut work_state), GameInput(input, restart)) => {
            if restart {
                return Ok((JustInited, Noop));
            }
            if work_state.world.send_to_player(input)
                == Err(input_system::InputSystemError::NoPlayerToSend)
            {
                warn!("{}", input_system::InputSystemError::NoPlayerToSend);
            }
            let new_state = Working(work_state);
            return Ok((new_state, Noop));
        }
        (Working(mut work_state), ProcessMainGamepadInput(None)) => {
            work_state.is_gamepad_connected = Some(false);
            let new_state = Working(work_state);
            return Ok((new_state, Noop));
        }
        (Working(mut work_state), ProcessMainGamepadInput(Some(gamepad))) => {
            work_state.is_gamepad_connected = Some(true);
            let new_state = Working(work_state);
            let (input, restart) = translate_gamepad(&gamepad);
            return Ok((new_state, GameInput(input, restart)));
        }
        (Working(work_state), TranslateRawEvent(ev)) => {
            if let Some(true) = &work_state.is_gamepad_connected {
                let new_state = Working(work_state);
                return Ok((new_state, Noop));
            }
            let new_state = Working(work_state);
            match translate_keyboard(ev) {
                None => {
                    return Ok((new_state, Noop));
                }
                Some(kbd_inpt) => {
                    return Ok((new_state, Keyboard(kbd_inpt)));
                }
            }
        }
        (Working(mut work_state), Keyboard(kbd)) => match kbd {
            KeyboardInput::QuitRequest => {
                let new_state = Working(work_state);
                return Ok((new_state, Quit));
            }
            KeyboardInput::GameRestart => {
                return Ok((JustInited, Noop));
            }
            KeyboardInput::GameInput(game_input_command) => {
                if work_state.world.send_to_player(vec![game_input_command])
                    == Err(input_system::InputSystemError::NoPlayerToSend)
                {
                    warn!("{}", input_system::InputSystemError::NoPlayerToSend);
                }
                let new_state = Working(work_state);
                return Ok((new_state, Noop));
            }
        },
        (_, Quit) => {
            return Ok((Exiting, Noop));
        }
        (state_out, _) => {
            return Ok((state_out, Noop));
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
