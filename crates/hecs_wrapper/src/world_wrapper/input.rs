use eyre::Result;
use std::collections::VecDeque;

use super::components::*;
//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub enum InputCommand {
    TypeDigital(u8),
    OnceUp,
    OnceDown,
    OnceLeft,
    OnceRight,
}

#[derive(Debug)]
struct CommandBuffer {
    commands: VecDeque<InputCommand>,
}
impl CommandBuffer {
    fn new() -> Self {
        Self {
            commands: VecDeque::new(),
        }
    }
    fn add_last(&mut self, cmd: InputCommand) {
        self.commands.push_back(cmd);
    }
    fn take_first(&mut self) -> Option<InputCommand> {
        self.commands.pop_front()
    }
}

//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn insert_input(&mut self, inputs: Vec<InputCommand>) -> Result<()> {
        let input_entity = match self.input_entity {
            Some(entity) => entity,
            None => {
                let new_entity = self.world.spawn(());
                self.world.insert_one(new_entity, CommandBuffer::new())?;
                new_entity
            }
        };
        self.input_entity = Some(input_entity);
        let mut input_holder = self
            .world
            .get::<&mut CommandBuffer>(self.input_entity.unwrap())?;

        for cmd in inputs.into_iter() {
                input_holder.add_last(cmd);
        }

        Ok(())
    }

    pub(crate) fn input_system_update(&mut self) -> Result<()> {
        let Some(input_entity) = self.input_entity else {
            return Ok(());
        };

        let mut di = 0.;
        let mut dj = 0.;
        {
            let mut input_holder = self.world.get::<&mut CommandBuffer>(input_entity)?;

            todo!("nesessary to extract command list. process it. and include creating new Entities with NUMBER");

            while let Some(cmd) = input_holder.take_first() {
                match cmd {
                    InputCommand::OnceUp => dj -= 1.,
                    InputCommand::OnceDown => dj += 1.,
                    InputCommand::OnceLeft => di -= 1.,
                    InputCommand::OnceRight => di += 1.,
                    _ => (),
                }
            }
        }

        for (_id, (position, _)) in self.world.query_mut::<(&mut Position, &UserInput)>() {
            *position += (di, dj).into();
        }

        Ok(())
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::super::*;
    use super::*;

    #[test]
    fn repopulation() -> Result<()> {
        let mut world = RaaWorld::new();
        assert!(world.world.len() == 0);
        world.repopulate();
        assert!(world.input_entity == None);
        let base_count = world.world.len();

        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.input_entity != None);
        assert!(world.world.len() == base_count + 1);
        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.input_entity != None);
        assert!(world.world.len() == base_count + 1);

        world.repopulate();
        assert!(world.input_entity == None);
        assert!(world.world.len() == base_count);

        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.input_entity != None);
        assert!(world.world.len() == base_count + 1);
        assert!(world.input_entity != None);
        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.world.len() == base_count + 1);
        Ok(())
    }

    #[test]
    fn creation() -> Result<()> {
        let mut world = RaaWorld::new();
        assert!(world.input_entity == None);
        assert!(world.world.len() == 0);

        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.input_entity != None);
        assert!(world.world.len() == 1);
        world.insert_input(vec![InputCommand::OnceUp])?;
        world.insert_input(vec![InputCommand::OnceUp])?;
        assert!(world.world.len() == 1);
        Ok(())
    }
}
