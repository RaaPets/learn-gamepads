use cells_world::*;
pub use player::PlayerInput;

//  //  //  //  //  //  //  //
pub struct CellType(pub CellState);

//  //  //  //  //  //  //  //
use arithm2d::pos2d;

pub struct CentralEntity;
pub type CellPosition = pos2d::Pos2D<isize>;

pub type Position = pos2d::Pos2D<f64>;
pub struct Movement(pub pos2d::Pos2D<f64>);
pub struct Velocity(pub pos2d::Pos2D<f64>);

//  //  //  //  //  //  //  //
pub mod player {
    use std::collections::VecDeque;
    //use crate::prelude::GameInputCommand;

    #[derive(Debug, PartialEq)]
    pub enum GameInputCommand {
        TypeDigital(char),
        OnceUp,
        OnceDown,
        OnceLeft,
        OnceRight,
        Accelerate((f32, f32)),
    }

    //  //  //  //  //  //  //
    pub struct PlayerInput {
        pub(crate) input_buffer: VecDeque<GameInputCommand>,
    }
    impl PlayerInput {
        pub(crate) fn new() -> Self {
            Self {
                input_buffer: VecDeque::new(),
            }
        }

        pub(crate) fn clear(&mut self) {
            self.input_buffer.clear();
        }
        pub(crate) fn add_last(&mut self, cmd: GameInputCommand) {
            self.input_buffer.push_back(cmd);
        }
        pub(crate) fn _take_first(&mut self) -> Option<GameInputCommand> {
            self.input_buffer.pop_front()
        }
    }
}
