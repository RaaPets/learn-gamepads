use arithm2d::pos2d::Pos2D;

use super::components::*;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn collision_system(&mut self) {
        let mut ent_list = Vec::new();

        for (id, _position) in self.world.query::<&Position>().iter() {
            ent_list.push(id);
        }

        for primary in 0..(ent_list.len() as isize - 1) {
            for secondary in (primary + 1)..ent_list.len() as isize {
                let ent_a = ent_list[primary as usize];
                let ent_b = ent_list[secondary as usize];

                if let Some((mov_a, mov_b)) =
                    check_entities_for_correction(&self.world, &ent_a, &ent_b)
                {
                    self.world.insert_one(ent_a, mov_a);
                    self.world.insert_one(ent_b, mov_b);
                }
            }
        }
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn check_entities_for_correction(
    world: &hecs::World,
    ent_a: &hecs::Entity,
    ent_b: &hecs::Entity,
) -> Option<(Movement, Movement)> {
    if ent_a == ent_b {
        return None;
    }
    let Ok(pos_a) = world.get::<&Position>(*ent_a) else {
        return None;
    };
    let Ok(pos_b) = world.get::<&Position>(*ent_b) else {
        return None;
    };
    check_positions_for_correction(&pos_a, &pos_b)
}

#[inline(always)]
fn check_positions_for_correction(
    pos_a: &Position,
    pos_b: &Position,
) -> Option<(Movement, Movement)> {
    let int_pos_a = Pos2D::<isize> {
        x: pos_a.x as isize,
        y: pos_a.y as isize,
    };
    let int_pos_b = Pos2D::<isize> {
        x: pos_b.x as isize,
        y: pos_b.y as isize,
    };
    if int_pos_a != int_pos_b {
        return None;
    }
    if pos_a == pos_b {
        let mov_a = Movement((-0.01, 0.).into());
        let mov_b = Movement((0.01, 0.).into());
        return Some((mov_a, mov_b));
    }
    let a_to_b = *pos_b - *pos_a;
    let mov_a = Movement(-a_to_b / 2.);
    let mov_b = Movement(a_to_b / 2.);
    return Some((mov_a, mov_b));
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod correct_bounds_test {
    use super::*;
    use crate::prelude::RaaWorld;

    #[test]
    fn simple_pre_validation_check() {
        let mut world = RaaWorld::new();
        let pos_a = world.world.spawn((1, Position::from((2., 2.))));
        let pos_b = world.world.spawn((2, Position::from((2.1, 2.))));
        let no_pos_a = world.world.spawn((3, true));
        let no_pos_b = world.world.spawn((4, false));

        assert!(check_entities_for_correction(&world.world, &pos_a, &no_pos_a).is_none());
        assert!(check_entities_for_correction(&world.world, &pos_b, &no_pos_b).is_none());
        assert!(check_entities_for_correction(&world.world, &no_pos_a, &no_pos_b).is_none());
        assert!(check_entities_for_correction(&world.world, &no_pos_a, &pos_b).is_none());
        assert!(check_entities_for_correction(&world.world, &no_pos_a, &pos_a).is_none());
        assert!(check_entities_for_correction(&world.world, &pos_a, &pos_a).is_none());
    }

    /*
    #[test]
    fn triple() {
        let mut world = RaaWorld::new();
        world.world.spawn((1, Position::from((2., 2.))));
        world.world.spawn((2, Position::from((2.1, 2.))));
        world.world.spawn((3, Position::from((5., 2.))));

        world.collision_system();
    }

    #[test]
    fn pair() {
        let mut world = RaaWorld::new();
        world.world.spawn((1, Position::from((2., 2.))));
        world.world.spawn((2, Position::from((2.1, 2.))));

        world.collision_system();
    }
    */

    #[test]
    fn single() {
        let mut world = RaaWorld::new();

        world.world.spawn((1, Position::from((2., 2.))));
        world.collision_system();

        world.world.spawn((true, 25));
        world.collision_system();
    }

    #[test]
    fn no_with_pos() {
        let mut world = RaaWorld::new();
        world.collision_system();
        world.world.spawn((true, 25));
        world.collision_system();
    }

    #[test]
    fn cross_index() {
        let ent_list_len = 0;
        for primary in 0..(ent_list_len - 1) {
            for secondary in (primary + 1)..ent_list_len {
                println!("{} <---> {}", primary, secondary);
            }
        }
    }
}
