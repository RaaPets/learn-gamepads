use cells_world::*;

//  //  //  //  //  //  //  //
pub struct UserInput;
pub struct CellType(pub CellState);

//  //  //  //  //  //  //  //
use arithm2d::pos2d;
pub type Position = pos2d::Pos2D<f64>;

//  //  //  //  //  //  //  //
pub mod player {
    use crate::prelude::InputCommand;
    use std::collections::VecDeque;

    pub struct PlayerInput {
        pub(crate) input_buffer: VecDeque<InputCommand>,
    }
    impl PlayerInput {
        pub(crate) fn new() -> Self {
            Self {
                input_buffer: VecDeque::new(),
            }
        }
    }
}
