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
        (AppState::Working(mut working), Action::GameInput(input)) => {
            working.world.process_input(&input)?;
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
            let input = translateGamepad(&gamepad);
            return Ok((new_state, Action::GameInput(input)));
        }
        (AppState::Working(working), Action::TranslateRawEvent(ev)) => {
            if let Some(true) = &working.is_gamepad_connected {
                let new_state = AppState::Working(working);
                return Ok((new_state, Action::Noop));
            }
            let input = translateKeyboard(ev);
            let new_state = AppState::Working(working);
            return Ok((new_state, Action::GameInput(input)));
        }
        (_, Action::Quit) => {
            return Ok((AppState::Exiting, Action::Noop));
        }
        (state_out, _) => {
            //warn!("unprocessed Action <{:?}>", action);
            return Ok((state_out, Action::Noop));
        }
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn translateGamepad(gamepad: &gamepads::Gamepad) -> WorldInput {
    let mut di = 0;
    let mut dj = 0;
    let mut restart = false;
    for button in gamepad.all_just_pressed() {
        match button {
            Button::DPadUp => dj = -1,
            Button::DPadDown => dj = 1,
            Button::DPadLeft => di = -1,
            Button::DPadRight => di = 1,
            Button::RightCenterCluster => restart = true,
            _ => (),
        }
    }

    WorldInput { di, dj, restart }
}

//  //  //  //  //  //  //  //
use ratatui::crossterm::event as xEvent;

#[inline(always)]
fn translateKeyboard(event: xEvent::Event) -> WorldInput {

    let mut di = 0;
    let mut dj = 0;
    let mut restart = false;
    if let xEvent::Event::Key(key) = event {
        /*
        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
            if key.code == xEvent::KeyCode::Char('p') {
                //return Ok(Action::PopupLuaEditor);
            }
        }
        */
            if key.code == xEvent::KeyCode::Char('k') {
                dj = -1;
            }
            if key.code == xEvent::KeyCode::Char('j') {
                dj = 1;
            }
            if key.code == xEvent::KeyCode::Char('h') {
                di = -1;
            }
            if key.code == xEvent::KeyCode::Char('l') {
                di = 1;
            }
            if key.code == xEvent::KeyCode::Char('r') {
                restart = true;
            }
    }

    WorldInput { di, dj, restart }
}
