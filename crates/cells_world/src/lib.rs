//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CellState {
    Empty,
    RedEmpty,
    Player,
    Target,
    Obstacle,
    SomeChar(char),
}

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub struct CellsWorld {
    pub width: usize,
    pub height: usize,
    cells2d: Box<[Box<[CellState]>]>,
}

impl CellsWorld {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells2d: vec![vec![CellState::Empty; width].into_boxed_slice(); height]
                .into_boxed_slice(),
        }
    }

    fn normalize_ij(&self, ij: (isize, isize)) -> (usize, usize) {
        let (src_i, src_j) = ij;
        let mut i = src_i % self.width as isize;
        let mut j = src_j % self.height as isize;

        if i < 0 {
            i += self.width as isize;
        }
        if j < 0 {
            j += self.height as isize;
        }

        (i as usize, j as usize)
    }
}

impl std::ops::Index<(usize, usize)> for CellsWorld {
    type Output = CellState;

    fn index(&self, uij: (usize, usize)) -> &Self::Output {
        let (ui, uj) = uij;
        let (i, j) = self.normalize_ij((ui as isize, uj as isize));
        &self.cells2d[j][i]
    }
}
impl std::ops::IndexMut<(usize, usize)> for CellsWorld {
    fn index_mut(&mut self, uij: (usize, usize)) -> &mut Self::Output {
        let (ui, uj) = uij;
        let (i, j) = self.normalize_ij((ui as isize, uj as isize));
        &mut self.cells2d[j][i]
    }
}

impl std::ops::Index<(isize, isize)> for CellsWorld {
    type Output = CellState;

    fn index(&self, ij: (isize, isize)) -> &Self::Output {
        let (i, j) = self.normalize_ij(ij);
        &self.cells2d[j][i]
    }
}
impl std::ops::IndexMut<(isize, isize)> for CellsWorld {
    fn index_mut(&mut self, ij: (isize, isize)) -> &mut Self::Output {
        let (i, j) = self.normalize_ij(ij);
        &mut self.cells2d[j][i]
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod cells_world {
    use super::*;

    #[test]
    fn normalize_example() {
        let width = 3;
        let height = 5;
        let mut world = CellsWorld::new(width, height);

        world[(1_usize, 1_usize)] = CellState::Obstacle;
        world[(0_usize, 0_usize)] = CellState::Player;
        world[(9_isize, 2_isize)] = CellState::RedEmpty;

        for i in 0..width {
            for j in 0..height {
                assert!(world[(i, j)] == world[(i + width, j)]);
                assert!(world[(i, j)] == world[(i + width, j + height)]);
                assert!(world[(i, j)] == world[(i, j + height)]);

                let i2 = i.overflowing_sub(10*width).0;
                let j2 = j.overflowing_sub(10*height).0;
                assert!(world[(i, j)] == world[(i2, j2)]);
            }
        }
    }

    #[test]
    fn normalize_ij_3() {
        let width = 3;
        let height = 3;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0,0) ) == (0,0));
        assert!(world.normalize_ij( (1,1) ) == (1,1));
        assert!(world.normalize_ij( (2,2) ) == (2,2));
        assert!(world.normalize_ij( (3,3) ) == (0,0));
        assert!(world.normalize_ij( (4,4) ) == (1,1));
        assert!(world.normalize_ij( (-1, -1) ) == (2,2));
        assert!(world.normalize_ij( (-2, -2) ) == (1,1));
        assert!(world.normalize_ij( (-3, -3) ) == (0,0));
    }
    #[test]
    fn normalize_ij_2() {
        let width = 2;
        let height = 2;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0,0) ) == (0,0));
        assert!(world.normalize_ij( (1,1) ) == (1,1));
        assert!(world.normalize_ij( (2,2) ) == (0,0));
        assert!(world.normalize_ij( (3,3) ) == (1,1));
        assert!(world.normalize_ij( (-1, -1) ) == (1,1));
        assert!(world.normalize_ij( (-2, -2) ) == (0,0));
    }

    #[test]
    fn normalize_j_2() {
        let width = 1;
        let height = 2;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0,0) ) == (0,0));
        assert!(world.normalize_ij( (0,1) ) == (0,1));
        assert!(world.normalize_ij( (0,2) ) == (0,0));
        assert!(world.normalize_ij( (0,3) ) == (0,1));
        assert!(world.normalize_ij( (0, -1) ) == (0,1));
        assert!(world.normalize_ij( (0, -2) ) == (0,0));
    }
    #[test]
    fn normalize_j_1() {
        let width = 1;
        let height = 1;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0, 0) ) == (0,0));
        assert!(world.normalize_ij( (0, 1) ) == (0,0));
        assert!(world.normalize_ij( (0, 2) ) == (0,0));
        assert!(world.normalize_ij( (0, -1) ) == (0,0));
        assert!(world.normalize_ij( (0, -2) ) == (0,0));
    }

    #[test]
    fn normalize_i_2() {
        let width = 2;
        let height = 1;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0,0) ) == (0,0));
        assert!(world.normalize_ij( (1,0) ) == (1,0));
        assert!(world.normalize_ij( (2,0) ) == (0,0));
        assert!(world.normalize_ij( (3,0) ) == (1,0));
        assert!(world.normalize_ij( (-1,0) ) == (1,0));
        assert!(world.normalize_ij( (-2,0) ) == (0,0));
    }
    #[test]
    fn normalize_i_1() {
        let width = 1;
        let height = 1;
        let world = CellsWorld::new(width, height);

        assert!(world.normalize_ij( (0,0) ) == (0,0));
        assert!(world.normalize_ij( (1,0) ) == (0,0));
        assert!(world.normalize_ij( (2,0) ) == (0,0));
        assert!(world.normalize_ij( (-1,0) ) == (0,0));
        assert!(world.normalize_ij( (-2,0) ) == (0,0));
    }

    #[test]
    fn one_mod() {
        let width = 2;
        let height = 5;
        let mut world = CellsWorld::new(width, height);

        world[(1_usize, 4_usize)] = CellState::Obstacle;

        assert!(world[(1_usize, 4_usize)] == CellState::Obstacle);
    }

    #[test]
    fn create_empty() {
        let width = 2;
        let height = 4;
        let world = CellsWorld::new(width, height);

        for i in 0..width {
            for j in 0..height {
                assert!(world[(i, j)] == CellState::Empty);
            }
        }
    }
}
