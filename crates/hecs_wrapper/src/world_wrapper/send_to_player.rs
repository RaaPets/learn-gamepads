use super::*;
use crate::error::input_system::*;

//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn send_to_player(&mut self, inputs: Vec<GameInputCommand>) -> Result {
        for (_id, player_input) in self.world.query_mut::<&mut player::PlayerInput>() {
            player_input.clear();
            for cmd in inputs.into_iter() {
                player_input.add_last(cmd);
            }
            return Ok(());
        }
        Err(InputSystemError::NoPlayerToSend)
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
        /*
    use super::*;
    use cells_world::*;

    #[test]
    fn creation() -> eyre::Result<()> {
        let mut world = World::new();
        assert!(world.world.len() == 0);
        invoke_player_input_update(&mut world.world);

        assert!(
            world.send_to_player(vec![GameInputCommand::OnceUp])
                == Err(InputSystemError::NoPlayerToSend)
        );
        invoke_player_input_update(&mut world.world);

        world
            .world
            .spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        assert!(world.world.len() == 1);
        assert!(
            world.send_to_player(vec![GameInputCommand::OnceUp])
                == Err(InputSystemError::NoPlayerToSend)
        );
        invoke_player_input_update(&mut world.world);

        world.world.spawn((
            CellType(CellState::Player),
            UserInput,
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
        ));
        assert!(world.world.len() == 2);
        world.send_to_player(vec![GameInputCommand::OnceUp])?;
        invoke_player_input_update(&mut world.world);

        Ok(())
    }
        */
}
