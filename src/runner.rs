use eyre::{Result, WrapErr};
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod action;
use action::Action;
mod updater;
mod viewer;

mod app_state;
use app_state::AppState;

static TICK: std::time::Duration = std::time::Duration::from_millis(300);

//  //  //  //  //  //  //  //
pub fn execute(
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    trace!("runner::execute()..");
    let mut app = AppState::JustInited;

    let handler = event_handler::EventHandler::new(TICK);
    let mut gamepad_handler = gamepad_handler::GamepadHandler::new();

    while !app.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&mut app, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      get inputs
        let raw_input = handler
            .wait_next()
            .wrap_err("handler.wait_next() in runner::execute()")?;
        //      process inputs
        match raw_input {
            event_handler::Events::Input(raw_event) => {
                invoke_update_loop(Action::TranslateRawEvent(raw_event), &mut app)?;
            }
            event_handler::Events::Tick => {
                gamepad_handler.update();
                let main_gamepad = gamepad_handler.get_gamepad(0);
                invoke_update_loop(Action::ProcessMainGamepadInput(main_gamepad), &mut app)?;

                invoke_update_loop(Action::Tick, &mut app)?;
            }
            event_handler::Events::Exit => {
                invoke_update_loop(Action::Quit, &mut app)?;
            }
        }
    }
    trace!("normal exit");
    Ok(())
}

//  //  //  //  //  //  //  //
fn invoke_update_loop(first_action: Action, app_state: &mut AppState) -> Result<()> {
    let mut current_action = first_action;
    while if let Action::Noop = current_action {
        false
    } else {
        true
    } {
        current_action = updater::update(app_state, &current_action)?;
    }
    Ok(())
}
