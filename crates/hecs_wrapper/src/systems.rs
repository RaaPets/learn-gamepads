use crate::components::*;

//  //  //  //  //  //  //  //
pub mod movement;
pub mod player_input;
pub mod velocity;

pub mod collision;

//  //  //  //  //  //  //  //
pub mod position_to_cell {
    use super::*;

    pub(crate) fn update(positions: hecs::QueryMut<(&mut CellPosition, &mut Position)>) {
        for (_id, (cell_pos, part_pos)) in positions {
            if part_pos.x >= 1. {
                cell_pos.x += 1;
                part_pos.x -= 1.;
            }
            if part_pos.y >= 1. {
                cell_pos.y += 1;
                part_pos.y -= 1.;
            }
            if part_pos.x <= -1. {
                cell_pos.x -= 1;
                part_pos.x += 1.;
            }
            if part_pos.y <= -1. {
                cell_pos.y -= 1;
                part_pos.y += 1.;
            }
        }
    }
}

//  //  //  //  //  //  //  //
pub mod center_on_position {
    use super::*;

    pub(crate) fn update(positions: hecs::QueryMut<&mut CellPosition>, position: Option<CellPosition>) {
        if let Some(central_cell) = position {
            for (_id, cell_pos) in positions {
                *cell_pos -= central_cell;
            }
        }
    }
}
