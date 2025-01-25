use eyre::Result;
use gamepads::Button;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::Action;
use super::AppState;
use super::WorkingParams;
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
            working.world.insert_input(input)?;
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
            let (input, restart) = translate_keyboard(ev);
            let new_state = AppState::Working(working);
            return Ok((new_state, Action::GameInput(input, restart)));
        }
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
fn translate_gamepad(gamepad: &gamepads::Gamepad) -> (Vec<InputCommand>, bool) {
    let mut restart = false;
    let mut cmds = Vec::new();
    for button in gamepad.all_just_pressed() {
        match button {
            Button::RightCenterCluster => restart = true,
            Button::DPadUp => cmds.push(InputCommand::OnceUp),
            Button::DPadDown => cmds.push(InputCommand::OnceDown),
            Button::DPadLeft => cmds.push(InputCommand::OnceLeft),
            Button::DPadRight => cmds.push(InputCommand::OnceRight),
            Button::ActionUp => cmds.push(InputCommand::TypeDigital(1),
            Button::ActionRight => cmds.push(InputCommand::TypeDigital(2),
            Button::ActionDown => cmds.push(InputCommand::TypeDigital(3),
            Button::ActionLeft => cmds.push(InputCommand::TypeDigital(4),
            _ => (),
        }
    }

    (cmds, restart)
}

//  //  //  //  //  //  //  //
use ratatui::crossterm::event as xEvent;

#[inline(always)]
fn translate_keyboard(event: xEvent::Event) -> (Vec<InputCommand>, bool) {
    let mut restart = false;
    let mut cmds = Vec::new();
    if let xEvent::Event::Key(key) = event {
            if key.code == xEvent::KeyCode::Char('k') {
            cmds.push(InputCommand::OnceUp);
            }
            if key.code == xEvent::KeyCode::Char('j') {
            cmds.push(InputCommand::OnceDown);
            }
            if key.code == xEvent::KeyCode::Char('h') {
            cmds.push(InputCommand::OnceLeft);
            }
            if key.code == xEvent::KeyCode::Char('l') {
            cmds.push(InputCommand::OnceRight);
            }
            if key.code == xEvent::KeyCode::Char('r') {
                restart = true;
            }
    }

    (cmds, restart)
}
