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

                touch_correction(&self.world, &ent_a, &ent_b);
            }
        }
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn touch_correction(
    world: &hecs::World,
    ent_a: &hecs::Entity,
    ent_b: &hecs::Entity,
) -> Option<bool> {
    if ent_a == ent_b {
        return None;
    }
    let Ok(pos_a) = world.get::<&Position>(*ent_a) else {
        return None;
    };
    let Ok(pos_b) = world.get::<&Position>(*ent_b) else {
        return None;
    };
    todo!("touch_correction");
}

#[inline(always)]
fn touch_correction(
    world: &hecs::World,
    ent_a: &hecs::Entity,
    ent_b: &hecs::Entity,
) -> Option<bool> {
    if ent_a == ent_b {
        return None;
    }
    let Ok(pos_a) = world.get::<&Position>(*ent_a) else {
        return None;
    };
    let Ok(pos_b) = world.get::<&Position>(*ent_b) else {
        return None;
    };
    todo!("touch_correction");
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

        assert!(touch_correction(&world.world, &pos_a, &no_pos_a) == None);
        assert!(touch_correction(&world.world, &pos_b, &no_pos_b) == None);
        assert!(touch_correction(&world.world, &no_pos_a, &no_pos_b) == None);
        assert!(touch_correction(&world.world, &no_pos_a, &pos_b) == None);
        assert!(touch_correction(&world.world, &no_pos_a, &pos_a) == None);
        assert!(touch_correction(&world.world, &pos_a, &pos_a) == None);
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
