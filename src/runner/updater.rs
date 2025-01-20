use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};
use gamepads::Button;

use super::app_state::TestPos;
use super::Action;
use super::AppState;

//  //  //  //  //  //  //  //
pub fn update(app_state: &mut AppState, action: &Action) -> Result<Action> {
    match action {
        Action::ProcessMainGamepadInput(opt_gamepad) => {
            let pre_test_pos = match app_state {
                AppState::Working(_, pos) => pos.clone(),
                _ => TestPos { x: 7, y: 7 },
            };
            if let Some(gamepad) = opt_gamepad {
                let mut new_test_pos = pre_test_pos.clone();
                for button in gamepad.all_just_pressed() {
                    match button {
                        Button::DPadUp => new_test_pos.y -= 1,
                        Button::DPadDown => new_test_pos.y += 1,
                        Button::DPadLeft => new_test_pos.x -= 1,
                        Button::DPadRight => new_test_pos.x += 1,
                        _ => (),
                    }
                }
                new_test_pos.normalize();
                trace!("gm pos:\n{:?}", new_test_pos);
                *app_state = AppState::Working(true, new_test_pos);
            } else {
                *app_state = AppState::Working(false, pre_test_pos);
            }
        }
        _ => {
            warn!("unprocessed Action <{:?}>", action)
            //return Err(eyre::eyre!("unprocessed Action <{:?}>", action));
        }
    }
    Ok(Action::Noop)
}

//  //  //  //  //  //  //  //

/*
                for button in gamepad.all_just_pressed() {
                    match button {
                        Button::DPadUp => println!("{:?}", button),
                        Button::DPadDown => println!("{:?}", button),
                        Button::DPadLeft => println!("{:?}", button),
                        Button::DPadRight => println!("{:?}", button),
                        _ => {
                            println!("other -> {:?}", button);
                        }
                    }
                }
*/
