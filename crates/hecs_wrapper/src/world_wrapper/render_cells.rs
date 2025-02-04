use std::rc::Rc;

use super::*;

use arithm2d::pos2d::Pos2D;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn render_cells(&self, width: isize, height: isize) -> Option<Rc<CellsWorld>> {
        let half_width = width / 2 - 1;
        let half_height = height / 2 - 1;
        let mut cells = CellsWorld::new(width as usize, height as usize);

        let mut visuals = self.world.query::<(&CellType, &CellPosition)>();
        for (_id, (cell_type, cell_pos)) in visuals.iter() {
            let CellPosition { x: i, y: j } = *cell_pos + CellPosition::new(half_width, half_height);
            if i >= 0 && i < width && j >= 0 && j < height {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }

    pub fn render_cells2222(&self, width: isize, height: isize) -> Option<Rc<CellsWorld>> {
        let half_width = width / 2 - 1;
        let half_height = height / 2 - 1;

        let mut cells = CellsWorld::new(width as usize, height as usize);

        for (_id, (cell_type, src_pos)) in &mut self.world.query::<(&CellType, &Position)>() {
            let Position { x, y } = *src_pos;
            let ix = (x+1000000.) as isize - 1000000;
            let jy = (y+1000000.) as isize - 1000000;
            let position = Pos2D {
                x: ix,
                y: jy,
            };
            let rel_pos = position + (half_width, half_height).into();
            let (i, j) = rel_pos.into();
            if i >= 0 && i < width && j >= 0 && j < height {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }
}
