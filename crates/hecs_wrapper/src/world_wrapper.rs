use crate::prelude::*;
use cells_world::*;

pub(crate) mod central_position;
mod create_entity;
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
        let mut input_query = hecs::PreparedQuery::<(&mut Position, &mut PlayerInput)>::default();
        let res_char = systems::player_input::update(input_query.query_mut(world));

        systems::collision::update(world.query_mut::<(&mut Movement, &CellPosition)>(), rnd);

        systems::velocity::update(
            world.query_mut::<(&mut Movement, &Velocity)>(),
            delta_time_secs,
        );
        systems::movement::update(world.query_mut::<(&mut Position, &mut Movement)>());

        systems::position_to_cell::update(world.query_mut::<(&mut CellPosition, &mut Position)>());

        let central_cell =
            central_position::get_from_first(world.query::<(&CentralEntity, &CellPosition)>());
        systems::center_on_position::update(world.query_mut::<&mut CellPosition>(), central_cell);
        if let Some(ch) = res_char {
            Self::spawn_char(world, ch, (0, -1));
        }
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
