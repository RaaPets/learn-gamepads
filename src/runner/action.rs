use ratatui::crossterm::event as xEvent;

use hecs_wrapper::prelude::*;

//  //  //  //  //  //  //  //
//#[derive(Debug, PartialEq)]
#[derive(Debug)]
pub enum Action {
    // main
    Noop,
    Quit,
    Tick,
    TranslateRawEvent(xEvent::Event),

    // append
    GameInput(WorldInput),
    ProcessMainGamepadInput(Option<gamepads::Gamepad>),
    /*
    ApplyEditedCode(bool), // is GameCode?
    ResetCounters,
    GameUpdate,
    HandleByEditor(xEvent::Event),
    PopupLuaEditor,
    QueueCommand(String),
    Warning(String),
    */
}
