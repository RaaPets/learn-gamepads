use eyre::Result;
use hecs::*;

use cells_world::*;

pub mod components;
pub mod input;
pub mod render;
mod utils;

use components::*;

//  //  //  //  //  //  //  //
pub struct RaaWorld {
    pub(crate) world: World,
    pub(crate) input_entity: Option<Entity>,
}

impl RaaWorld {
    pub fn new() -> Self {
        let world = World::new();

        Self {
            world,
            input_entity: None,
        }
    }

    pub fn repopulate(&mut self) {
        let world = &mut self.world;

        self.input_entity = None;
        world.clear();

        let _player = world.spawn((
            CellType(CellState::Player),
            UserInput,
            Position::from_tuple((7, 7)),
        ));
        let _target1 = world.spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        let _obstacle = world.spawn((CellType(CellState::Obstacle), Position::from_tuple((1, 1))));
        let _target2 = world.spawn((CellType(CellState::Target), Position::from_tuple((19, 19))));
        let _target2 = world.spawn((
            CellType(CellState::Obstacle),
            Position::from_tuple((15, 15)),
        ));
    }

    pub fn update_on_tick(&mut self) -> Result<()> {
        self.input_system_update()?;

        //moving_system_update();

        //despawn ToDespawn

        Ok(())
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    fn repopulate() {
        let mut world = RaaWorld::new();
        assert!(world.world.len() == 0);
        world.repopulate();
        assert!(world.world.len() == 5);
        world.repopulate();
        assert!(world.world.len() == 5);
    }

    #[test]
    fn debug_print() {
        let mut world = RaaWorld::new();
        world.repopulate();
        println!("debug print\n{:?}", world);
    }
}
