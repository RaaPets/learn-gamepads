use super::*;

//mod unic_pairs;
//  //  //  //  //  //  //  //
pub(crate) fn update(items: hecs::QueryMut<(&mut Movement, &CellPosition)>, counter: u64) {
    let mut list: Vec<_> = items.into_iter().collect();
    let n = list.len();
    if n < 2 {
        return;
    }
    for alpha in 0..(n - 1) {
        for betta in alpha..n {
            let pos = list[alpha].1 .1;
            let counter_pos = list[betta].1 .1;
            let CellPosition { x, y } = *pos - *counter_pos;
            let delta = Position::new(x as f64, y as f64);
            let scalar_x = delta.x.abs();
            let scalar_y = delta.y.abs();
            let scalar = if scalar_x > scalar_y {
                scalar_x
            } else {
                scalar_y
            };
            if scalar >= 1. {
                continue;
            } else {
                if scalar > 0. {
                    list[alpha].1 .0 .0 += delta;
                    list[betta].1 .0 .0 -= delta;
                } else {
                    let r_delta = rnd_delta(counter);
                    list[alpha].1 .0 .0 += r_delta;
                    list[betta].1 .0 .0 -= r_delta;
                }
            }
        }
    }
}

const O: f64 = 0.0;
const I: f64 = 0.95;
fn rnd_delta(rnd: u64) -> Position {
    match rnd & 7 {
        0 => (O, I).into(),
        1 => (I, I).into(),
        2 => (I, O).into(),
        3 => (I, -I).into(),
        4 => (O, -I).into(),
        5 => (-I, -I).into(),
        6 => (-I, O).into(),
        _ => (-I, I).into(),
    }
}
/*
//use arithm2d::pos2d::Pos2D;
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

    #[test]
    fn simple_pre_validation_check() {
        let mut world = hecs::World::new();
        let pos_a = world.spawn((1, Position::from((2., 2.))));
        let pos_b = world.spawn((2, Position::from((2.1, 2.))));
        let no_pos_a = world.spawn((3, true));
        let no_pos_b = world.spawn((4, false));

        assert!(does_pair_interract(&world, &pos_a, &no_pos_a, 0).is_none());
        assert!(does_pair_interract(&world, &pos_b, &no_pos_b, 0).is_none());
        assert!(does_pair_interract(&world, &no_pos_a, &no_pos_b, 0).is_none());
        assert!(does_pair_interract(&world, &no_pos_a, &pos_b, 0).is_none());
        assert!(does_pair_interract(&world, &no_pos_a, &pos_a, 0).is_none());
        assert!(does_pair_interract(&world, &pos_a, &pos_a, 0).is_none());
    }

    #[test]
    fn single() {
        let mut world = hecs::World::new();

        world.spawn((1, Position::from((2., 2.))));
        update(&mut world, 1);

        world.spawn((true, 25));
        update(&mut world, 2);
    }

    #[test]
    fn no_with_pos() {
        let mut world = hecs::World::new();
        update(&mut world, 1);
        world.spawn((true, 25));
        update(&mut world, 2);
    }
}
*/
