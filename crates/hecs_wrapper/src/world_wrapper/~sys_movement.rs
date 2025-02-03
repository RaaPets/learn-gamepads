use super::components::*;

static ZERO: Position = Position { x: 0., y: 0. };
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn move_system_update(&mut self) -> Option<Position> {
        let mut first_player_position = None;

        for (_id, (position, movement)) in self.world.query_mut::<(&mut Position, &Movement)>() {
            *position += movement.0;
            if first_player_position.is_none() {
                first_player_position = Some(*position);
            }
        }

        for (_id, movement) in self.world.query_mut::<&mut Movement>() {
            movement.0 = ZERO;
        }

        first_player_position
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
/*
#[cfg(test)]
mod base_test {
    use super::*;
    use crate::prelude::*;
    use crate::world_wrapper::*;

    #[test]
    fn creation() -> eyre::Result<()> {
        let mut world = RaaWorld::new();
        assert!(world.world.len() == 0);

        world.move_system_update();
        invoke_player_input_update(&mut world.world);
        world.move_system_update();

        world
            .world
            .spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        assert!(world.world.len() == 1);
        invoke_player_input_update(&mut world.world);
        world.move_system_update();

        world.world.spawn((
            CellType(CellState::Player),
            UserInput,
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
        ));
        assert!(world.world.len() == 2);
        world.send_to_player(vec![GameInputCommand::OnceUp])?;
        invoke_player_input_update(&mut world.world);
        world.move_system_update();

        Ok(())
    }
}
*/
