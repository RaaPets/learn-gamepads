use std::hash::Hash;
use std::rc::Rc;

//  //  //  //  //  //  //  //
#[derive(Debug, Hash, Eq)]
pub struct Entity;
impl Entity {
    pub(crate) fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base {
    use super::*;

    #[test]
    fn not_eq_another() {
        let ent1 = Entity {};
        let ent2 = Entity {};
        assert!(ent1 != ent2);
    }

    #[test]
    fn eq_itself() {
        let ent1 = Entity {};
        assert!(ent1 == ent1);
    }
}
