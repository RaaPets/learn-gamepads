use crate::prelude::*;
use cells_world::*;

mod render_cells;
mod send_to_player;
mod utils;
//  //  //  //  //  //  //  //
pub struct RaaWorld {
    counter: u64,
    pub(crate) world: hecs::World,
}

impl RaaWorld {
    pub fn new() -> Self {
        let world = hecs::World::new();
        Self { counter: 0, world }
    }

    pub fn update_on_tick(&mut self) -> eyre::Result<()> {
        self.counter += 1;
        Self::update_world(&mut self.world, self.counter);
        Ok(())
    }

    #[inline(always)]
    fn update_world(world: &mut hecs::World, counter: u64) {
        let player_position =
            systems::player_input::update(world.query_mut::<(&mut Position, &PlayerInput)>());
        systems::player_input::clear(world.query_mut::<&mut PlayerInput>());

        systems::collision::update(world.query_mut::<(&mut Movement, &Position)>(), counter);
        //systems::collision::update(world, counter);

        systems::movement::update(world.query_mut::<(&mut Position, &mut Movement)>());

        systems::center_on_position::update(world.query_mut::<&mut Position>(), player_position);
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
impl RaaWorld {
    pub fn repopulate(&mut self) {
        let world = &mut self.world;

        world.clear();

        let _target0 = world.spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        let _char = world.spawn((
            CellType(CellState::SomeChar('2')),
            Position::from_tuple((1, 1)),
            Movement(Position::from((0., 0.))),
        ));
        let _player = world.spawn((
            CellType(CellState::Player),
            player::PlayerInput::new(),
            Position::from_tuple((7, 7)),
            Movement(Position::from((0., 0.))),
        ));
        let _target1 = world.spawn((
            CellType(CellState::Target),
            Position::from_tuple((19, 19)),
            Movement(Position::from((0., 0.))),
        ));
        let _obstacle = world.spawn((
            CellType(CellState::Obstacle),
            Position::from_tuple((15, 15)),
            Movement(Position::from_tuple((0, 0))),
        ));
    }
}
