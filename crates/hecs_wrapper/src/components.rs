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
use num_complex::Complex32;
pub struct WaveFunction {
    frz: f32,
    ampl: Complex32,
}
impl WaveFunction {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let phase = rng.random_range(0.0..1.);
        let frz = rng.random_range(0.5..5.0);
        Self {
            frz,
            ampl: Complex32::cis(phase),
        }
    }

    pub fn evo(&mut self, d: f32) {
        let dwave = Complex32::cis(d*self.frz);
        self.ampl *= dwave;
    }
     pub fn prob(&self) -> f32 {
         let (_, theta) = self.ampl.to_polar();
         theta.cos()
     }
     pub fn interferr(lhr: &Self, rhr: &Self) -> (f64, f64) {
         let (_, theta0) = lhr.ampl.to_polar();
         let (_, theta2) = rhr.ampl.to_polar();
         let theta: f64 = (theta0+theta2).into();
         (theta.cos(), theta.sin())
     }
}

//  //  //  //  //  //  //  //
pub mod player {
    use std::collections::VecDeque;

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
