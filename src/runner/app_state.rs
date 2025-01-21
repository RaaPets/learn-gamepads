use std::rc::Rc;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub enum AppState {
    JustInited,
    Working(bool, Rc<cells_world::CellsWorld>),
    Exiting,
}

impl AppState {
    pub fn is_exiting(&self) -> bool {
        *self == Self::Exiting
    }
}

