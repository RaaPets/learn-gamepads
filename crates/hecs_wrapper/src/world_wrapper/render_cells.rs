use super::*;
use std::rc::Rc;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn render_cells(&self, width: isize, height: isize) -> Option<Rc<CellsWorld>> {
        let mut cells = CellsWorld::new(width as usize, height as usize);
        for i in 0..width {
            for j in 0..height {
                cells[(i, j)] = match self.space[(i, j)] {
                    EntityCell::Empty => CellState::Empty,
                    EntityCell::Entity(ent) => ent_cell_state(&self.world, ent),
                    EntityCell::EntityAnd(ent, _) => CellState::RedEmpty,//ent_cell_state(&self.world, ent),
                };
            }
        }

        Some(Rc::new(cells))
    }
}

fn ent_cell_state(world: &hecs::World, ent: hecs::Entity) -> CellState {
    match world.get::<&CellType>(ent) {
        Ok(ctype) => ctype.0,
        _ => CellState::RedEmpty,
    }
}
