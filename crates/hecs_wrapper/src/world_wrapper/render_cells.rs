use super::*;
use std::rc::Rc;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn render_cells(&self, width: isize, height: isize) -> Option<Rc<CellsWorld>> {
        let half_width = width / 2 - 1;
        let half_height = height / 2 - 1;
        let mut cells = CellsWorld::new(width as usize, height as usize);

        let mut visuals = self.world.query::<(&CellType, &CellPosition)>();
        for (_id, (cell_type, cell_pos)) in visuals.iter() {
            let CellPosition { x: i, y: j } =
                *cell_pos + CellPosition::new(half_width, half_height);
            if i >= 0 && i < width && j >= 0 && j < height {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }
}
