use super::*;

//  //  //  //  //  //  //  //
impl PartialEq for RaaWorld {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl std::fmt::Debug for RaaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.world.len() {
            0 => write!(f, "there is no entity"),
            1 => write!(f, "there is 1 entity"),
            n => write!(f, "there are {} entities", n),
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    fn a_few() {
        let mut world = RaaWorld::new();
        world.world.spawn( (true, 25));
        world.world.spawn( (4., 5));
        world.world.spawn( (false, 25.));

        let s = "there are 3 entities";
        let r = format!("{:?}", world);
        assert!(r == s);
    }

    #[test]
    fn single() {
        let mut world = RaaWorld::new();
        world.world.spawn( (true, 25));

        let s = "there is 1 entity";
        let r = format!("{:?}", world);
        assert!(r == s);
    }

    #[test]
    fn empty() {
        let world = RaaWorld::new();

        let s = "there is no entity";
        let r = format!("{:?}", world);
        assert!(r == s);
    }
}
