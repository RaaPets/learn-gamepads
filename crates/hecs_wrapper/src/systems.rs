use crate::components::*;

//  //  //  //  //  //  //  //
pub mod movement;
pub mod player_input;
pub mod velocity;

pub mod collision;

//  //  //  //  //  //  //  //
pub mod pos_to_space {
    use super::*;
    use crate::world_wrapper::EntityCell;
    use cells_space::CellsSpace;

    pub(crate) fn update(
        space: &mut CellsSpace<EntityCell>,
        mut cposes: hecs::PreparedQueryIter<&CellPosition>,
    ) {
        space.clear();
        for (id, pos) in cposes {
            let s_pos = pos.into_tuple::<isize>();
            match &mut space[s_pos] {
                EntityCell::Empty => {
                    space[pos.into_tuple::<isize>()] = EntityCell::Entity(id);
                },
                EntityCell::Entity(ent_root)=> {
                    space[pos.into_tuple::<isize>()] = EntityCell::EntityAnd(*ent_root, vec![id]);
                },
                EntityCell::EntityAnd(_, appends)=> {
                    appends.push(id)
                },
            }
        }
    }
}

//  //  //  //  //  //  //  //
pub mod wave_function {
    use super::*;

    pub(crate) fn update(waves: hecs::PreparedQueryIter<&mut WaveFunction>, dfaze: f32) {
        for (_id, wave) in waves {
            wave.evo(dfaze);
        }
    }
}

//  //  //  //  //  //  //  //
pub mod position_to_cell {
    use super::*;

    pub(crate) fn update(positions: hecs::QueryMut<(&mut CellPosition, &mut Position)>) {
        for (_id, (cell_pos, part_pos)) in positions {
            process_position(cell_pos, part_pos);
        }
    }

    fn process_position(cell_pos: &mut CellPosition, part_pos: &mut Position) {
        if part_pos.x >= 1. {
            part_pos.x -= 1.;
            cell_pos.x += 1;
        }
        if part_pos.y >= 1. {
            part_pos.y -= 1.;
            cell_pos.y += 1;
        }
        if part_pos.x <= -1. {
            part_pos.x += 1.;
            cell_pos.x -= 1;
        }
        if part_pos.y <= -1. {
            part_pos.y += 1.;
            cell_pos.y -= 1;
        }
    }

    //  //  //  //  //  //  //  //
    //        TEST              //
    //  //  //  //  //  //  //  //
    #[cfg(test)]
    mod process_position_test {
        use super::*;

        #[test]
        fn changed_2() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(-2.9, -2.9);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(1, 5));
            assert!(part_pos == Position::new(-1.9, -1.9));
        }
        #[test]
        fn changed_1() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(2.9, 2.9);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(3, 7));
            assert!(part_pos == Position::new(1.9, 1.9));
        }

        #[test]
        fn changed_yy_a2() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(0.2, -1.7);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(2, 5));
            assert!(part_pos == Position::new(0.2, -0.7));
        }
        #[test]
        fn changed_yy_a1() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(0.2, 1.7);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(2, 7));
            assert!(part_pos == Position::new(0.2, 0.7));
        }

        #[test]
        fn changed_xx_a_2() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(-1.7, -0.4);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(1, 6));
            assert!(part_pos == Position::new(-0.7, -0.4));
        }
        #[test]
        fn changed_xx_a_1() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(1.7, -0.4);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(3, 6));
            assert!(part_pos == Position::new(0.7, -0.4));
        }

        #[test]
        fn changed_yy_2() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(0.2, -1.);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(2, 5));
            assert!(part_pos == Position::new(0.2, 0.));
        }
        #[test]
        fn changed_yy_1() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(0.2, 1.);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(2, 7));
            assert!(part_pos == Position::new(0.2, 0.));
        }

        #[test]
        fn changed_xx_2() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(-1., -0.4);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(1, 6));
            assert!(part_pos == Position::new(0., -0.4));
        }
        #[test]
        fn changed_xx_1() {
            let mut cell_pos = CellPosition::new(2, 6);
            let mut part_pos = Position::new(1., -0.4);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(3, 6));
            assert!(part_pos == Position::new(0., -0.4));
        }

        #[test]
        fn not_changed_3() {
            let mut cell_pos = CellPosition::new(5, 6);
            let mut part_pos = Position::new(0.2, -0.4);

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(5, 6));
            assert!(part_pos == Position::new(0.2, -0.4));
        }

        #[test]
        fn not_changed_2() {
            let mut cell_pos = CellPosition::new(5, 6);
            let mut part_pos = Position::default();

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos == CellPosition::new(5, 6));
            assert!(part_pos == Position::default());
        }

        #[test]
        fn not_changed_1() {
            let mut cell_pos = CellPosition::default();
            let mut part_pos = Position::default();

            process_position(&mut cell_pos, &mut part_pos);

            assert!(cell_pos.x == 0);
            assert!(cell_pos.y == 0);
            assert!(part_pos.x == 0.);
            assert!(part_pos.y == 0.);
        }

        #[test]
        fn chek_defaults() {
            let cell_pos = CellPosition::default();
            let part_pos = Position::default();
            assert!(cell_pos.x == 0);
            assert!(cell_pos.y == 0);
            assert!(part_pos.x == 0.);
            assert!(part_pos.y == 0.);
        }
    }
}

//  //  //  //  //  //  //  //
use arithm2d::pos2d::Pos2D;
const fix_centr: Pos2D<isize> = Pos2D{ x: 7, y: 7 };

pub mod center_on_position {
    use super::*;

    pub(crate) fn update(
        positions: hecs::QueryMut<&mut CellPosition>,
        position: Option<CellPosition>,
    ) {
        if let Some(central_cell) = position {
            for (_id, cell_pos) in positions {
                *cell_pos -= central_cell;
                *cell_pos += fix_centr;
            }
        }
    }
}
