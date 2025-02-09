use ratatui::crossterm::event as xEvent;

use hecs_wrapper::prelude::*;

#[derive(Debug, PartialEq)]
pub enum KeyboardInput {
    GameRestart,
    GameInput(GameInputCommand),
    GameQuitRequest,
}
//  //  //  //  //  //  //  //
#[derive(Debug)]
pub enum Action {
    // main
    Noop,
    AppQuit,
    Tick,
    TranslateRawEvent(xEvent::Event),
    Keyboard(KeyboardInput),

    // append
    GameInput(Vec<GameInputCommand>, bool),
    ProcessMainGamepadInput(Option<gamepads::Gamepad>),
}
