use eyre::Result;

use cells_world::*;

//  //  //  //  //  //  //  //
#[derive(Debug)]
pub struct WorldInput {
    pub di: isize,
    pub dj: isize,
    pub restart: bool,
}

//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn process_input(&mut self, input: &WorldInput) -> Result<()> {
        let WorldInput {
            di,
            dj,
            restart: false,
        } = input
        else {
            self.cells_cache = None;
            return Ok(());
        };
        for (_id, (position, cell_type)) in self.world.query_mut::<(&mut super::Position, &super::CellType, )>() {
            if let CellState::Player = cell_type.0 {
            } else {
                position.0 -= di;
                position.1 -= dj;
            }
        }

        Ok(())
    }
}
