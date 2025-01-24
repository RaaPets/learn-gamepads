use eyre::Result;
use hecs::*;
use std::rc::Rc;

use cells_world::*;

pub mod input;

//  //  //  //  //  //  //  //
struct CellType(CellState);
struct UserInput;
struct Position(isize, isize);

//  //  //  //  //  //  //  //
pub struct RaaWorld {
    pub(crate) world: World,
    //cells_cache: Option<CellsWorld>,
}

impl RaaWorld {
    pub fn new() -> Self {
        let mut world = World::new();
        let _player = world.spawn((CellType(CellState::Player), UserInput, Position(7, 7)));
        let _target1 = world.spawn((CellType(CellState::Target), Position(2, 2)));
        let _target2 = world.spawn((CellType(CellState::Target), Position(9, 5)));
        let _obstacle = world.spawn((CellType(CellState::Obstacle), Position(12, 15)));

        Self {
            world,
            //cells_cache: None,
        }
    }

    pub fn update_on_tick(&mut self) -> Result<()> {
        /*
        if let Some(_cells) = &self.cells_cache {
        } else {
            let cells = generate_cells(15, 15);
            self.cells_cache = Some(cells);
        }
        */
        Ok(())
    }

    pub fn render_cells(&self, width: usize, height: usize) -> Option<Rc<CellsWorld>> {
        let mut cells = CellsWorld::new(width, height);

        for (_id, (cell_type, position)) in &mut self.world.query::<(&CellType, &Position)>() {
                    cells[(position.0, position.1)] = cell_type.0;
        }

        Some(Rc::new(cells));
    }
}

impl PartialEq for RaaWorld {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl std::fmt::Debug for RaaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RaaWorld has {} entities", self.world.len())
    }
}

//  //  //  //  //  //  //  //
fn generate_cells(width: usize, height: usize) -> CellsWorld {
    let mut cells = CellsWorld::new(width, height);

    cells[(2_isize, 2)] = CellState::Target;
    cells[(9_isize, 5)] = CellState::Target;
    cells[(12_isize, 15)] = CellState::Obstacle;
    //cells[(7_isize, 7)] = CellState::Player;

    cells
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    fn debug_print() {
        let world = RaaWorld::new();
        println!("debug print\n{:?}", world);
    }
}
