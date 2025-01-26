use eyre::Result;
use hecs::*;

use cells_world::*;

pub mod components;
pub mod input;
pub mod movement;
pub mod render;
mod utils;

use components::*;

//  //  //  //  //  //  //  //
pub struct RaaWorld {
    pub(crate) world: World,
}

impl RaaWorld {
    pub fn new() -> Self {
        let world = World::new();

        Self { world }
    }

    pub fn repopulate(&mut self) {
        let world = &mut self.world;

        world.clear();

        let _player = world.spawn((
            CellType(CellState::Player),
            UserInput,
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
        ));
        let _target0 = world.spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        let _char = world.spawn((CellType(CellState::SomeChar('2')), Position::from_tuple((1, 1))));
        let _target1 = world.spawn((CellType(CellState::Target), Position::from_tuple((19, 19))));
        let _obstacle = world.spawn((
            CellType(CellState::Obstacle),
            Position::from_tuple((15, 15)),
        ));
    }

    pub fn update_on_tick(&mut self) -> Result<()> {
        self.input_system_update()?;

        self.movement_system_update();

        //despawn ToDespawn

        Ok(())
    }
}
