use eyre::Result;
use hecs::*;

use cells_world::*;

pub mod components;
pub mod sys_input;
pub mod sys_player_movement;
pub mod sys_collision;
pub mod sys_render;
mod utils;

use components::*;

//  //  //  //  //  //  //  //
pub struct RaaWorld {
    counter: u64,
    pub(crate) _space_size: i32,
    pub(crate) world: World,
}

impl RaaWorld {
    pub fn new() -> Self {
        Self::with_size(16)
    }

    pub fn with_size(space_size: u16) -> Self {
        let world = World::new();

        Self {
            counter: 0,
            _space_size: space_size.into(),
            world,
        }
    }

    pub fn update_on_tick(&mut self) -> Result<()> {
        self.counter += 1;

        self.input_system_update()?;

        self.move_player_system_update();

        self.collision_system_update();

        let _ = self.pre_rendering_system_update();

        //despawn ToDespawn

        Ok(())
    }
}


//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
impl RaaWorld {
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
        let _char = world.spawn((
            CellType(CellState::SomeChar('2')),
            Position::from_tuple((1, 1)),
        ));
        let _target1 = world.spawn((CellType(CellState::Target), Position::from_tuple((19, 19))));
        let _obstacle = world.spawn((
            CellType(CellState::Obstacle),
            Position::from_tuple((15, 15)),
        ));
    }
}
