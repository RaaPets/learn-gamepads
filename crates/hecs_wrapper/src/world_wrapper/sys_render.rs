use std::rc::Rc;

use super::*;

use arithm2d::pos2d::Pos2D;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn render_cells(&self, width: usize, height: usize) -> Option<Rc<CellsWorld>> {
        let mut player_entity = None;
        for (id, (_position, _cell_type)) in self
            .world
            .query::<(&Position, &UserInput)>().iter()
        {
            player_entity = Some(id);
        }
        let Some(player_entity) = player_entity else {
            return None;
        };
        let player_position = self.world.get::<&Position>(player_entity).ok()?;
        let center = Pos2D { x: player_position.x as isize, y: player_position.y as isize };
        let half_width = (width / 2) as isize - 1;
        let half_height = (height / 2) as isize - 1;

        let mut cells = CellsWorld::new(width, height);

        for (_id, (cell_type, src_pos)) in &mut self.world.query::<(&CellType, &Position)>() {
            let position = Pos2D { x: src_pos.x as isize, y: src_pos.y as isize };
            let rel_pos = position - center + (half_width, half_height).into();
            let (i, j) = rel_pos.into();
            if i >= 0 && i < (width as isize) && j>= 0 && j < (height as isize) {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }
}
