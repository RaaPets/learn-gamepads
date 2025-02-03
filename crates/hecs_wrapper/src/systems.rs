use crate::components::*;

//  //  //  //  //  //  //  //
pub mod player_input;
pub mod movement;
pub mod collision;

//  //  //  //  //  //  //  //
pub mod center_on_position {
    use super::*;

    pub(crate) fn update(positions: hecs::QueryMut<&mut Position>, position: Option<Position>) {
        if let Some(center_position) = position {
            for (_id, position) in positions {
                *position -= center_position;
            }
        }
    }
}
