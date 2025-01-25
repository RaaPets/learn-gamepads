use std::rc::Rc;

use super::*;

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
        let center = self.world.get::<&Position>(player_entity).ok()?;
        let half_width = ((width / 2) as isize - 1) as f64;
        let half_height = ((height / 2) as isize - 1) as f64;

        let mut cells = CellsWorld::new(width, height);

        for (_id, (cell_type, position)) in &mut self.world.query::<(&CellType, &Position)>() {
            let rel_pos = *position - *center + (half_width, half_height).into();
            let i = rel_pos.x as isize;
            let j = rel_pos.y as isize;
            if i >= 0 && i < (width as isize) && j>= 0 && j < (height as isize) {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }
}
