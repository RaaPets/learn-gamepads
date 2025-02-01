use eyre::{Result, WrapErr};
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod action;
use action::Action;
mod updater;
mod viewer;

mod app_state;
use app_state::*;

static TICK: std::time::Duration = std::time::Duration::from_millis(20);

//  //  //  //  //  //  //  //
pub fn execute(
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    trace!("runner::execute()..");

    let handler = event_handler::EventHandler::new(TICK, true);
    let mut gamepad_handler = gamepad_handler::GamepadHandler::new();

    let mut app_state = AppState::JustInited;
    while !&app_state.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&app_state, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      get inputs
        let raw_input = handler
            .wait_next()
            .wrap_err("handler.wait_next() in runner::execute()")?;
        //      process inputs
        match raw_input {
            event_handler::Events::Input(raw_event) => {
                app_state = invoke_update_loop(app_state, Action::TranslateRawEvent(raw_event))?;
            }
            event_handler::Events::Tick => {
                let main_gamepad = {
                    gamepad_handler.update();
                    gamepad_handler.get_gamepad(0)
                };
                app_state =
                    invoke_update_loop(app_state, Action::ProcessMainGamepadInput(main_gamepad))?;

                app_state = invoke_update_loop(app_state, Action::Tick)?;
            }
            event_handler::Events::Exit => {
                app_state = invoke_update_loop(app_state, Action::Quit)?;
            }
        }
    }
    trace!("normal exit");
    Ok(())
}

//  //  //  //  //  //  //  //
fn invoke_update_loop(first_state: AppState, first_action: Action) -> Result<AppState> {
    let mut current: (AppState, Action) = (first_state, first_action);

    while if let (_, Action::Noop) = current {
        false
    } else {
        true
    } {
        current = updater::update(current.0, current.1)?;
    }

    Ok(current.0)
}
