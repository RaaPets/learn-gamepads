use crate::error::input_system::*;

use super::components::*;
//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub enum InputCommand {
    TypeDigital(char),
    OnceUp,
    OnceDown,
    OnceLeft,
    OnceRight,
    Accelerate((f32, f32)),
}
//  //  //  //  //  //  //  //
mod with_player {
    use super::*;
    pub(super) fn add_last(player_input: &mut player::PlayerInput, cmd: InputCommand) {
        player_input.input_buffer.push_back(cmd);
    }
    pub(super) fn take_first(player_input: &mut player::PlayerInput) -> Option<InputCommand> {
        player_input.input_buffer.pop_front()
    }
}
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn send_to_player(&mut self, inputs: Vec<InputCommand>) -> Result {
        for (_id, player_input) in self.world.query_mut::<&mut player::PlayerInput>() {
            for cmd in inputs.into_iter() {
                with_player::add_last(player_input, cmd);
            }
            return Ok(());
        }
        Err(InputSystemError::NoPlayerToSend)
    }

    pub(crate) fn input_system_update(&mut self) -> eyre::Result<()> {
        let mut player_entity = None;
        for (id, _) in self.world.query::<&player::PlayerInput>().iter() {
            player_entity = Some(id);
            break;
        }
        let Some(input_entity) = player_entity else {
            return Ok(());
        };

        let mut di = 0.;
        let mut dj = 0.;
        let mut last_ch = None;
        {
            let mut player_input = self.world.get::<&mut player::PlayerInput>(input_entity)?;

            while let Some(cmd) = with_player::take_first(&mut player_input) {
                match cmd {
                    InputCommand::OnceUp => dj -= 1.,
                    InputCommand::OnceDown => dj += 1.,
                    InputCommand::OnceLeft => di -= 1.,
                    InputCommand::OnceRight => di += 1.,
                    InputCommand::Accelerate((ddi, ddj)) => {
                        di += (ddi as f64) / 3.5;
                        dj -= (ddj as f64) / 3.5;
                    }
                    InputCommand::TypeDigital(ch) => last_ch = Some(ch),
                }
            }
        }
        if let Some(ch) = last_ch {
            let mut pos = (*self.world.get::<&Position>(input_entity)?).clone();
            pos.y = pos.y - 1.;
            self.world
                .spawn((CellType(cells_world::CellState::SomeChar(ch)), pos));
        }

        self.world
            .insert_one(input_entity, Movement((di, dj).into()))?;

        Ok(())
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
        world.input_system_update()?;

        assert!(
            world.send_to_player(vec![InputCommand::OnceUp])
                == Err(InputSystemError::NoPlayerToSend)
        );
        world.input_system_update()?;

        world
            .world
            .spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        assert!(world.world.len() == 1);
        assert!(
            world.send_to_player(vec![InputCommand::OnceUp])
                == Err(InputSystemError::NoPlayerToSend)
        );
        world.input_system_update()?;

        world.world.spawn((
            CellType(CellState::Player),
            UserInput,
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
        ));
        assert!(world.world.len() == 2);
        world.send_to_player(vec![InputCommand::OnceUp])?;
        world.input_system_update()?;

        Ok(())
    }
}
