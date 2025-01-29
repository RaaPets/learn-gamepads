use ratatui::crossterm::event as xEvent;

use hecs_wrapper::prelude::*;

#[derive(Debug, PartialEq)]
pub enum KeyboardInput {
    GameRestart,
    GameInput(GameInputCommand),
    QuitRequest,
}
//  //  //  //  //  //  //  //
#[derive(Debug)]
pub enum Action {
    // main
    Noop,
    Quit,
    Tick,
    TranslateRawEvent(xEvent::Event),
    Keyboard(KeyboardInput),

    // append
    GameInput(Vec<GameInputCommand>, bool),
    ProcessMainGamepadInput(Option<gamepads::Gamepad>),

    /*
    PopupLuaEditor,
    QueueCommand(String),
    Warning(String),
    */
}
