use crate::prelude::*;
use cells_world::*;

pub(crate) mod central_position;
mod render_cells;
mod send_to_player;
mod utils;
//  //  //  //  //  //  //  //
pub struct RaaWorld {
    t: f64,
    pub(crate) world: hecs::World,
}

impl RaaWorld {
    pub fn new() -> Self {
        let world = hecs::World::new();
        Self { t: 0., world }
    }

    pub fn update_on_tick(&mut self, delta_time_secs: f64) -> eyre::Result<()> {
        self.t += delta_time_secs;
        Self::update_world(&mut self.world, delta_time_secs, (self.t * 123.) as u64);
        Ok(())
    }

    #[inline(always)]
    fn update_world(world: &mut hecs::World, delta_time_secs: f64, rnd: u64) {
        let _res_char =
            systems::player_input::update(world.query_mut::<(&mut Position, &mut PlayerInput)>());

        /*
        if let Some((pos, Some(ch))) = after_input {
        let new_pos = pos + Position::from((0., -1.));
        let _ = world.spawn((
            CellType(CellState::SomeChar(ch)),
            new_pos,
            Movement(Position::from((0., 0.))),
            Velocity(Position::from((0., 0.0))),
        ));
        }
        */

        systems::collision::update(world.query_mut::<(&mut Movement, &Position)>(), rnd);

        //systems::velocity::update(world.query_mut::<(&mut Movement, &Velocity)>(), delta_time_secs);
        systems::movement::update(world.query_mut::<(&mut Position, &mut Movement)>());

        systems::position_to_cell::update(world.query_mut::<(&mut CellPosition, &mut Position)>());

        //let central_cell = find_center_cell(&world);
        let central_cell = central_position::get_from_first(world.query::<(&CentralEntity, &CellPosition)>());
        systems::center_on_position::update(world.query_mut::<&mut CellPosition>(), central_cell);
    }

    pub fn debug_info(&self) -> String {
        let mut info = String::new();

        for ent in self.world.iter() {
            let cell_pos = ent.get::<&CellPosition>();
            let pos = ent.get::<&Position>();
            let e = ent.entity();
            info += &format!("\n[{}]-> {:?} {:?}", e.id(), cell_pos, pos);
        }

        info
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
            player::PlayerInput::new(),
            CellPosition::new(7, 7),
            Position::default(),
            Movement(Position::default()),
        ));

        let _target0 = world.spawn((CellType(CellState::Target), Position::from_tuple((0, 0))));
        let _char = world.spawn((
            CellType(CellState::SomeChar('2')),
            CellPosition::new(1, 1),
            Position::default(),
            Movement(Position::default()),
            Velocity(Position::new(0., 1.)),
        ));
        let _target1 = world.spawn((
            CellType(CellState::Target),
            Position::from_tuple((19, 19)),
            Movement(Position::default()),
        ));
        let _obstacle = world.spawn((
            CellType(CellState::Obstacle),
            CellPosition::new(15, 15),
            Position::default(),
            Movement(Position::default()),
        ));
    }
}
