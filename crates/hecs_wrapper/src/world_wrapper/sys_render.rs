use std::rc::Rc;

use super::*;
use crate::error::pre_rendering_system;

use arithm2d::pos2d::Pos2D;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn pre_rendering_system_update(&mut self) -> pre_rendering_system::Result {

        // TODO: needs tests. at least simplest

        let mut player_entity = None;
        for (id, (_position, _cell_type)) in self
            .world
            .query::<(&Position, &UserInput)>().iter()
        {
            player_entity = Some(id);
        }
        let Some(player_entity) = player_entity else {
            return Err(pre_rendering_system::PreRenderingSystemError::NoPlayerForCentering);
        };
        let player_position: Position = {
            let Ok(position) = self.world.get::<&Position>(player_entity) else {
                    return Err(pre_rendering_system::PreRenderingSystemError::PlayerHasNoPosition);
            };
            *position
        };

        for (_id, position) in self.world.query_mut::<&mut Position>() {
            *position -= player_position;
        }

        Ok(())
    }

    pub fn render_cells(&self, width: isize, height: isize) -> Option<Rc<CellsWorld>> {
        let half_width = width / 2 - 1;
        let half_height = height / 2 - 1;

        let mut cells = CellsWorld::new(width as usize, height as usize);

        for (_id, (cell_type, src_pos)) in &mut self.world.query::<(&CellType, &Position)>() {
            let position = Pos2D { x: src_pos.x as isize, y: src_pos.y as isize };
            let rel_pos = position + (half_width, half_height).into();
            let (i, j) = rel_pos.into();
            if i >= 0 && i < width && j>= 0 && j < height {
                cells[(i, j)] = cell_type.0;
            }
        }

        Some(Rc::new(cells))
    }
}
