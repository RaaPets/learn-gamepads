use super::*;
//  //  //  //  //  //  //  //
pub(crate) fn get_from_first(
    mut candidates: hecs::QueryBorrow<(&CentralEntity, &CellPosition)>,
) -> Option<super::CellPosition> {
    for (_id, (_, cell_pos)) in &mut candidates {
        return Some(*cell_pos);
    }
    None
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod player_input_test {
    use super::*;

    fn invoke_get_from_first(world: &hecs::World) -> Option<super::CellPosition> {
        get_from_first(world.query::<(&CentralEntity, &CellPosition)>())
    }

    #[test]
    fn check_success() {
        let mut world = hecs::World::new();
        {
            let p = invoke_get_from_first(&world);
            assert!(p == None);
        }

        let _ = world.spawn((false, CellPosition::new(7, 6), CentralEntity, 1));
        {
            let p = invoke_get_from_first(&world);
            assert!(p == Some(CellPosition::new(7, 6)));
        }
    }

    #[test]
    fn check_for_none() {
        let mut world = hecs::World::new();
        {
            let p = invoke_get_from_first(&world);
            assert!(p == None);
        }

        let _ent_a = world.spawn((false, 1));
        {
            let p = invoke_get_from_first(&world);
            assert!(p == None);
        }
        let _ent_b = world.spawn((false, CellPosition::default()));
        let _ent_c = world.spawn((false, CentralEntity));
        {
            let p = invoke_get_from_first(&world);
            assert!(p == None);
        }
    }
}
