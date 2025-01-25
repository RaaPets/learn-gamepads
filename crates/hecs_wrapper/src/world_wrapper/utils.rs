//  //  //  //  //  //  //  //
impl PartialEq for super::RaaWorld {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl std::fmt::Debug for super::RaaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RaaWorld has {} entities", self.world.len())
    }
}

