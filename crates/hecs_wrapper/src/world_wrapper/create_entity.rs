use super::*;
use hecs::{World, Entity};

impl super::RaaWorld {
    pub fn repopulate(&mut self) {
        let world = &mut self.world;

        world.clear();

        Self::spawn_player(world, (7, 7));

        Self::spawn_target(world, (0, 0));
        Self::spawn_char(world, '4', (1, 1));
        Self::spawn_target(world, (19, 19));
        Self::spawn_obstacle(world, (15, 15));
    }

    pub(crate) fn spawn_player(world: &mut World, pos: (isize, isize)) {
        let ent = world.spawn((
            CellType(CellState::Player),
            player::PlayerInput::new(),
            CentralEntity,
        ));
        Self::insert_location_components(world, ent, pos);
    }
    pub(crate) fn spawn_char(world: &mut World, ch: char, pos: (isize, isize)) {
        let ent = world.spawn((
            CellType(CellState::SomeChar(ch)),
            //Velocity(Position::new(0., 1.)),
        ));
        Self::insert_location_components(world, ent, pos);
    }
    pub(crate) fn spawn_target(world: &mut World, pos: (isize, isize)) {
        let ent = world.spawn((
            CellType(CellState::Target),
        ));
        Self::insert_location_components(world, ent, pos);
    }
    pub(crate) fn spawn_obstacle(world: &mut World, pos: (isize, isize)) {
        let ent = world.spawn((
            CellType(CellState::Obstacle),
        ));
        Self::insert_location_components(world, ent, pos);
    }

    fn insert_location_components(world: &mut World, into_entity: Entity, pos: (isize, isize)) {
        let _ = world.insert(into_entity, (
            CellPosition::from(pos),
            Position::default(),
            Movement(Position::default()),
        ));
    }

}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {}
