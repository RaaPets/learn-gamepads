use std::rc::Rc;
use eyre::Result;
use gamepads::Button;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::Action;
use super::AppState;
use cells_world::*;

//  //  //  //  //  //  //  //
pub fn update(app_state: &mut AppState, action: &Action) -> Result<Action> {
    match (&app_state, action) {
        (_, Action::Quit) => {
            *app_state = AppState::Exiting;
            return Ok(Action::Noop);
        }
        (AppState::Working(_, world), Action::ProcessMainGamepadInput(None)) => {
            *app_state = AppState::Working(false, world.clone());
            return Ok(Action::Noop);
        }
        (AppState::Working(_, old_world), Action::ProcessMainGamepadInput(Some(gamepad))) => {
            let (di, dj, restart) = translateGamepad(gamepad);
            if restart {
                *app_state = AppState::JustInited;
                return Ok(Action::Noop);
            }
            if di == 0 && dj == 0 {
                *app_state = AppState::Working(true, old_world.clone());
            } else {
                let mut new_world = CellsWorld::new(old_world.width, old_world.height);
                let mut old_player = None;
                for i in 0..new_world.width as isize {
                    for j in 0..new_world.height as isize {
                        let old_i = i + di;
                        let old_j = j + dj;
                        let old_cell = old_world[(old_i, old_j)];
                        if old_cell == CellState::Player {
                            new_world[(i, j)] = CellState::Empty;
                            old_player = Some((old_i, old_j));
                        } else {
                            new_world[(i, j)] = old_cell;
                        }
                    }
                }
                if let Some((pi, pj)) = old_player {
                        new_world[(pi, pj)] = CellState::Player;
                }
                *app_state = AppState::Working(true, Rc::new(new_world));
            }
            return Ok(Action::Noop);
        }
        (AppState::JustInited, Action::Tick) => {
            let started_world = create_world();
            *app_state = AppState::Working(false, started_world);
            return Ok(Action::Noop);
        }
        _ => {
            //warn!("unprocessed Action <{:?}>", action);
            return Ok(Action::Noop);
        }
    }
}

//  //  //  //  //  //  //  //
fn create_world() -> Rc<CellsWorld> {
    let mut creation = CellsWorld::new(15, 15);

    creation[(2_isize,2)] = CellState::Target;
    creation[(9_isize,5)] = CellState::Target;
    creation[(12_isize,15)] = CellState::Obstacle;
    creation[(7_isize,7)] = CellState::Player;

    Rc::new(creation)
}

//  //  //  //  //  //  //  //
fn translateGamepad(gamepad: &gamepads::Gamepad) -> (isize, isize, bool) {
    let mut dx = 0;
    let mut dy = 0;
    let mut restart = false;
    for button in gamepad.all_just_pressed() {
        match button {
            Button::DPadUp => dy = -1,
            Button::DPadDown => dy = 1,
            Button::DPadLeft => dx = -1,
            Button::DPadRight => dx = 1,
            Button::RightCenterCluster => restart = true,
            _ => (),
        }
    }

    (dx, dy, restart)
}
