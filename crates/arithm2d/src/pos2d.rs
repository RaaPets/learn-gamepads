use std::ops;

//  //  //  //  //  //  //  //
#[derive(Debug, Copy, Clone)]
pub struct Pos2D<V> {
    pub x: V,
    pub y: V,
}
//  //  //  //  //  //  //  //

//  //  //  //  //  //  //  //
impl<V: std::cmp::PartialEq> PartialEq for Pos2D<V> {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        false
    }
}

//  //  //  //  //  //  //  //
impl<V: ops::SubAssign> ops::SubAssign for Pos2D<V> {
    fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
    }
}

impl<V: ops::AddAssign> ops::AddAssign for Pos2D<V> {
    fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
    }
}

//  //  //  //  //  //  //  //
impl<V: ops::Mul<Output = V> + ops::Add<Output = V>> ops::Mul for Pos2D<V> {
    type Output = V;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

//  //  //  //  //  //  //  //
impl<V: ops::Neg<Output = V>> ops::Neg for Pos2D<V> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl<V: ops::Sub<Output = V>> ops::Sub for Pos2D<V> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<V: ops::Add<Output = V>> ops::Add for Pos2D<V> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

//  //  //  //  //  //  //  //
impl<V> Pos2D<V> {
    pub fn from_tuple<T: Into<V>>(src: (T, T)) -> Self {
        let (x, y) = src;
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<V> From<(V, V)> for Pos2D<V> {
    fn from(src: (V, V)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

//  //  //  //  //  //  //  //
impl<V> Pos2D<V> {
    pub fn into_tuple<T: From<V>>(self) -> (T, T) {
        (self.x.into(), self.y.into())
    }
}

impl<V> Into<(V, V)> for Pos2D<V> {
    fn into(self) -> (V, V) {
        (self.x, self.y)
    }
}

//  //  //  //  //  //  //  //
impl<V: Default> Default for Pos2D<V> {
    fn default() -> Self {
        Self {
            x: V::default(),
            y: V::default(),
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod copmaration_tests {
    use super::*;

    #[test]
    fn not_eq3() {
        let pos1 = Pos2D::from((2, 1));
        let pos2 = Pos2D::from((1, 2));
        assert!(pos1 != pos2);
    }
    #[test]
    fn not_eq2() {
        let pos1 = Pos2D::from((2, 1));
        let pos2 = Pos2D::from((2, 3));
        assert!(pos1 != pos2);
    }
    #[test]
    fn not_eq1() {
        let pos1 = Pos2D::from((1, 3));
        let pos2 = Pos2D::from((2, 3));
        assert!(pos1 != pos2);
    }

    #[test]
    fn eq() {
        let pos1 = Pos2D::from((2, 3));
        let pos2 = Pos2D::from((2, 3));
        assert!(pos1 == pos2);
    }
}

#[cfg(test)]
mod arithm_tests {
    use super::*;

    #[test]
    fn sub_assign() {
        let mut pos1 = Pos2D::from((5, 9));
        let pos2 = Pos2D::from((4, 5));
        pos1 -= pos2;
        assert!(pos1.x == 1);
        assert!(pos1.y == 4);
    }
    #[test]
    fn add_assign() {
        let mut pos1 = Pos2D::from((5, 9));
        let pos2 = Pos2D::from((4, 5));
        pos1 += pos2;
        assert!(pos1.x == 9);
        assert!(pos1.y == 14);
    }

    #[test]
    fn scalar_mul() {
        let pos1 = Pos2D::from((2, 3));
        let pos2 = Pos2D::from((4, 5));
        let scalar = pos1 * pos2;
        assert!(scalar == 23);
    }

    #[test]
    fn neg() {
        let pos1 = Pos2D::from((5.3, -9.));
        let pos = -pos1;
        assert!(pos.x == -5.3);
        assert!(pos.y == 9.);
    }
    #[test]
    fn sub() {
        let pos1 = Pos2D::from((5, 9));
        let pos2 = Pos2D::from((4, 5));
        let pos = pos1 - pos2;
        assert!(pos.x == 1);
        assert!(pos.y == 4);
    }
    #[test]
    fn add() {
        let pos1 = Pos2D::from((2, 3));
        let pos2 = Pos2D::from((4, 5));
        let pos = pos1 + pos2;
        assert!(pos.x == 6);
        assert!(pos.y == 8);
    }
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn from_tuple() {
        let pos: Pos2D<f64> = Pos2D::from_tuple((2, 3));
        assert!(pos.x == 2.);
        assert!(pos.y == 3.);
    }
    #[test]
    fn from() {
        let pos = Pos2D::from((2, 3));
        assert!(pos.x == 2);
        assert!(pos.y == 3);
    }

    #[test]
    fn into2() {
        let pos = Pos2D { x: true, y: false };
        let tup: (usize, usize) = pos.into_tuple();
        assert!(tup.0 == 1);
        assert!(tup.1 == 0);
    }
    #[test]
    fn into() {
        let pos = Pos2D { x: 2, y: 3 };
        let tup: (isize, isize) = pos.into();
        assert!(tup.0 == 2);
        assert!(tup.1 == 3);
    }

    #[test]
    fn default() {
        let pos = Pos2D::<isize>::default();
        assert!(pos.x == 0);
        assert!(pos.y == 0);
    }
}
