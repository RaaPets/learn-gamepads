use super::components::*;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn movement_system_update(&mut self) {
        for (_id, (position, movement)) in self.world.query_mut::<(&mut Position, &Movement)>() {
            *position += movement.0;
        }

        for (_id, movement) in self.world.query_mut::<&mut Movement>() {
            movement.0 = (0., 0.).into();
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::*;
    use crate::prelude::*;
    use cells_world::*;

    #[test]
    fn creation() -> eyre::Result<()> {
        let mut world = RaaWorld::new();
        assert!(world.world.len() == 0);

        world.movement_system_update();
        world.input_system_update()?;
        world.movement_system_update();

        world
            .world
            .spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        assert!(world.world.len() == 1);
        world.input_system_update()?;
        world.movement_system_update();

        world.world.spawn((
            CellType(CellState::Player),
            UserInput,
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
        ));
        assert!(world.world.len() == 2);
        world.send_to_player(vec![GameInputCommand::OnceUp])?;
        world.input_system_update()?;
        world.movement_system_update();

        Ok(())
    }
}
