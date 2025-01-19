use ratatui::crossterm::event as xEvent;

//#[derive(Debug, PartialEq)]
#[derive(Debug)]
pub enum Action {
    // main
    Noop,
    Quit,
    Tick,
    TranslateRawEvent(xEvent::Event),

    // append
    ProcessMainGamepadInput(gamepads::Gamepad),
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
