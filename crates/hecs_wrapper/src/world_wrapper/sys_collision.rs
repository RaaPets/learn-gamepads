use arithm2d::pos2d::Pos2D;

use super::components::*;
//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn collision_system_update(&mut self) {
        let unic_paris = {
            let mut ent_list = Vec::new();

            for (id, _position) in self.world.query::<&Position>().iter() {
                ent_list.push(id);
            }

            into_unic_pairs(&ent_list)
        };

        /*
        for primary in 0..(ent_list.len() as isize - 1) {
            for secondary in (primary + 1)..ent_list.len() as isize {
                let ent_a = ent_list[primary as usize];
                let ent_b = ent_list[secondary as usize];

                if let Some((mov_a, mov_b)) =
                    does_pair_interract(&self.world, &ent_a, &ent_b, self.counter)
                {
                    let _ = self.world.insert_one(ent_a, mov_a);
                    let _ = self.world.insert_one(ent_b, mov_b);

                    // TODO: calc_interraction
                    //todo!("2) calc_interraction");
                }
            }
        }
        */
    }
}

//  //  //  //  //  //  //  //
fn into_unic_pairs(list: &[hecs::Entity]) -> Vec<(hecs::Entity, hecs::Entity)> {
    let mut pairs = Vec::new();

    for primary in 0..(list.len() as isize - 1) {
        for secondary in (primary + 1)..list.len() as isize {
            let ent_a = list[primary as usize];
            let ent_b = list[secondary as usize];
            if ent_a != ent_b {
                pairs.push((ent_a, ent_b));
            }
        }
    }

    pairs
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod into_unic_pairs_test {
    use super::*;

    #[test]
    fn triple() {
        let mut world = hecs::World::new();
        let a = world.spawn((false, 2));
        let b = world.spawn((false, 2));
        let c = world.spawn((false, 2));
        {
            let list = world
                .iter()
                .map(|ent_ref| ent_ref.entity())
                .collect::<Vec<_>>();
            let pairs = into_unic_pairs(&list);
            assert!(pairs.len() == 3);
        }
    }

    #[test]
    fn couple() {
        let mut world = hecs::World::new();
        let a = world.spawn((false, 2));
        let b = world.spawn((false, 2));
        {
            let list = [a, b];
            let pairs = into_unic_pairs(&list);
            assert!(pairs.len() == 1);
        }
        {
            let list = world
                .iter()
                .map(|ent_ref| ent_ref.entity())
                .collect::<Vec<_>>();
            let pairs = into_unic_pairs(&list);
            assert!(pairs.len() == 1);
        }
    }

    #[test]
    fn single() {
        let mut world = hecs::World::new();
        let a = world.spawn((false, 2));
        {
            let list = [a];
            let pairs = into_unic_pairs(&list);
            assert!(pairs.is_empty());
        }
        {
            let list = [a, a];
            let pairs = into_unic_pairs(&list);
            assert!(pairs.is_empty());
        }
        {
            let list = [a, a, a];
            let pairs = into_unic_pairs(&list);
            assert!(pairs.is_empty());
        }
    }

    #[test]
    fn empty() {
        let list: Vec<hecs::Entity> = vec![];
        let pairs = into_unic_pairs(&list);
        assert!(pairs.is_empty());
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn does_pair_interract(
    world: &hecs::World,
    ent_a: &hecs::Entity,
    ent_b: &hecs::Entity,
    rnd: u64,
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
    check_positions_for_correction(&pos_a, &pos_b, rnd)
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn check_positions_for_correction(
    pos_a: &Position,
    pos_b: &Position,
    rnd: u64,
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
        return Some(rnd_movement(rnd));
    }
    let a_to_b = *pos_b - *pos_a;
    let mov_a = Movement(-a_to_b / 2.);
    let mov_b = Movement(a_to_b / 2.);
    return Some((mov_a, mov_b));
}

fn rnd_movement(rnd: u64) -> (Movement, Movement) {
    let pos: Position = match rnd & 7 {
        0 => (0.0, 0.1).into(),
        1 => (0.1, 0.1).into(),
        2 => (0.1, 0.0).into(),
        3 => (0.1, -0.1).into(),
        4 => (0.0, -0.1).into(),
        5 => (-0.1, -0.1).into(),
        6 => (-0.1, 0.0).into(),
        _ => (-0.1, 0.1).into(),
    };

    (Movement(pos), Movement(-pos))
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

        assert!(does_pair_interract(&world.world, &pos_a, &no_pos_a, 0).is_none());
        assert!(does_pair_interract(&world.world, &pos_b, &no_pos_b, 0).is_none());
        assert!(does_pair_interract(&world.world, &no_pos_a, &no_pos_b, 0).is_none());
        assert!(does_pair_interract(&world.world, &no_pos_a, &pos_b, 0).is_none());
        assert!(does_pair_interract(&world.world, &no_pos_a, &pos_a, 0).is_none());
        assert!(does_pair_interract(&world.world, &pos_a, &pos_a, 0).is_none());
    }

    #[test]
    fn single() {
        let mut world = RaaWorld::new();

        world.world.spawn((1, Position::from((2., 2.))));
        world.collision_system_update();

        world.world.spawn((true, 25));
        world.collision_system_update();
    }

    #[test]
    fn no_with_pos() {
        let mut world = RaaWorld::new();
        world.collision_system_update();
        world.world.spawn((true, 25));
        world.collision_system_update();
    }
}
