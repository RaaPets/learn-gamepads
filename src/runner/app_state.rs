
//  //  //  //  //  //  //  //
#[derive(Debug, Clone, PartialEq)]
pub struct TestPos {
    pub x: u16,
    pub y: u16,
}
impl TestPos {
    pub fn normalize(&mut self) {
        self.x &= 0xF;
        self.y &= 0xF;
    }
}

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub enum AppState {
    JustInited,
    Working(bool, TestPos),
    Exiting,
}

impl AppState {
    pub fn is_exiting(&self) -> bool {
        *self == Self::Exiting
    }
}

