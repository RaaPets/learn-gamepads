use crate::prelude::GameInputCommand;
use super::components::*;

//  //  //  //  //  //  //  //

impl super::RaaWorld {
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

            while let Some(cmd) = player_input.take_first() {
                match cmd {
                    GameInputCommand::OnceUp => dj -= 1.,
                    GameInputCommand::OnceDown => dj += 1.,
                    GameInputCommand::OnceLeft => di -= 1.,
                    GameInputCommand::OnceRight => di += 1.,
                    GameInputCommand::Accelerate((ddi, ddj)) => {
                        di += (ddi as f64) / 3.5;
                        dj -= (ddj as f64) / 3.5;
                    }
                    GameInputCommand::TypeDigital(ch) => last_ch = Some(ch),
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
