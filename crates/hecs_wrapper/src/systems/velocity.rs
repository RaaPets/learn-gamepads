use super::*;

//static ZERO: Position = Position { x: 0., y: 0. };
//  //  //  //  //  //  //  //
pub(crate) fn update(items: hecs::QueryMut<(&mut Movement, &Velocity)>, time: f64) {
    for (_id, (movement, velocity)) in items {
        movement.0 = velocity.0 * time;
    }
}

/*
//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::*;

    fn invoke_update(world: &mut hecs::World) {
        update(world.query_mut::<(&mut Position, &mut Movement)>());
    }

    #[test]
    fn simple_move() -> eyre::Result<()> {
        let mut world = hecs::World::new();
        let ent_a = world.spawn((false, 1));
        let ent_b = world.spawn((false, 1));
        let pos_a = Position::from((2., 5.));
        let _ = world.insert_one(ent_a, pos_a);
        let pos_b = Position::from((5., 2.));
        let _ = world.insert_one(ent_b, pos_b);
        {
            invoke_update(&mut world);
            assert!(*world.get::<&Position>(ent_a)? == pos_a);
            assert!(*world.get::<&Position>(ent_b)? == pos_b);
        }
        {
            let mov_a = Movement(Position::from((-1., 2.)));
            let _ = world.insert_one(ent_a, mov_a);
            invoke_update(&mut world);
            let pos_a2 = Position::from((1., 7.));
            assert!(*world.get::<&Position>(ent_a)? == pos_a2);
            assert!(*world.get::<&Position>(ent_b)? == pos_b);
        }
        {
            invoke_update(&mut world);
            let pos_a2 = Position::from((1., 7.));
            assert!(*world.get::<&Position>(ent_a)? == pos_a2);
            assert!(*world.get::<&Position>(ent_b)? == pos_b);
        }

        Ok(())
    }
}
*/
