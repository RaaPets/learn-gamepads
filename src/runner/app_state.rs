use hecs_wrapper::prelude::*;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub struct WorkingParams {
    pub is_gamepad_connected: Option<bool>,
    pub world: Box<RaaWorld>,
}

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub enum AppState {
    JustInited,
    Working(WorkingParams),
    Exiting,
}

impl AppState {
    pub fn is_exiting(&self) -> bool {
        *self == Self::Exiting
    }
}

