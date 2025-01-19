use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::AppState;
use super::Action;

//  //  //  //  //  //  //  //
pub fn update(_app_state: &mut AppState, action: &Action) -> Result<Action> {
    match action {
        _ => warn!("unprocessed Action <{:?}>", action),
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
