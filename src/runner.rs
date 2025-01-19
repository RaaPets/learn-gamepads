use eyre::{Result, WrapErr};
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};
use gamepads::{Gamepads, Button};

use app_raa_tui::App;

static TICK: std::time::Duration = std::time::Duration::from_millis(10);

//  //  //  //  //  //  //  //
pub fn execute(
    app: &mut App,
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    trace!("runner::execute()..");
    let handler = event_handler::EventHandler::new(TICK);
    let mut gamepads = Gamepads::new();

    while !app.is_exiting() {
        // DRAW
        //terminal.draw(|frame| viewer::view(&mut app, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      get inputs
        gamepads.poll();
        for gamepad in gamepads.all() {
            if gamepad.id().value() == 0 {
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
            }
        }
        let raw_input = handler
            .wait_next()
            .wrap_err("handler.wait_next() in runner::execute()")?;
        //      process inputs
        match raw_input {
            event_handler::Events::Input(raw_event) => {
                //invoke_update_loop(Action::TranslateRawEvent(raw_event), &mut app)?;
            }
            event_handler::Events::Tick => {
                //invoke_update_loop(Action::Tick, &mut app)?;
            }
            event_handler::Events::Exit => {
                //invoke_update_loop(Action::Quit, &mut app)?;
                app.set_exiting();
            }
        }
    }
    trace!("normal exit");
    Ok(())
}
