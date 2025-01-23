use eyre::{Result, eyre};
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use std::collections::HashMap;
use std::rc::Rc;

use crate::entity::Entity;

//  //  //  //  //  //  //  //
#[derive(Debug)]
pub struct ComponentSet<Component: Default> {
    components: HashMap<Rc<Entity>, Rc<Component>>,
}

impl<Component: Default> ComponentSet<Component> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn remove_from(&mut self, entity: &Rc<Entity>) -> Result<()> {
        if let None = self.components.remove(entity) {
            return Err(eyre!("impossible to remove the component because the entity has NO the component"));
        }
        Ok(())
    }

    pub fn add_to(&mut self, entity: &Rc<Entity>) -> Result<Rc<Component>> {
        if self.components.contains_key(entity) {
            return Err(eyre!("impossible add the component because the entity already has the component"));
        }
        let new_component = Rc::new(Component::default());
        self.components.insert(entity.clone(), new_component.clone());
        Ok(new_component)
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }
}


//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base {
    use super::*;

    struct Comp1;
    impl Default for Comp1 {
        fn default() -> Self {
            Self
        }
    }
    //  //  //  //  //  //  //


    #[test]
    fn delete_entity() -> Result<()> {
        let mut compset= ComponentSet::<Comp1>::new();
        assert!(compset.size() == 0);

        let ent1 = Entity::new();
        let ent2 = Entity::new();
        let ent3 = Entity::new();
        compset.add_to(&ent1)?;
        compset.add_to(&ent2)?;
        compset.add_to(&ent3)?;
        assert!(compset.size() == 3);

        compset.remove_from(&ent2)?;
        assert!(compset.size() == 2);
        compset.add_to(&ent2)?;
        assert!(compset.size() == 3);
        compset.remove_from(&ent2)?;
        assert!(compset.size() == 2);

        Ok(())
    }

    #[test]
    fn add_3entity() -> Result<()> {
        let mut compset= ComponentSet::<Comp1>::new();
        assert!(compset.size() == 0);

        let ent1 = Entity::new();
        let ent2 = Entity::new();
        let ent3 = Entity::new();
        compset.add_to(&ent1)?;
        compset.add_to(&ent2)?;
        compset.add_to(&ent3)?;
        assert!(compset.size() == 3);

        Ok(())
    }

    #[test]
    fn twice_add_1entity() -> Result<()> {
        let mut compset= ComponentSet::<Comp1>::new();
        assert!(compset.size() == 0);

        let ent1 = Entity::new();
        compset.add_to(&ent1)?;
        assert!(compset.add_to(&ent1).is_err());
        assert!(compset.size() == 1);

        Ok(())
    }

    #[test]
    fn add_1entity() -> Result<()> {
        let mut compset= ComponentSet::<Comp1>::new();
        assert!(compset.size() == 0);

        let ent1 = Entity::new();
        compset.add_to(&ent1)?;
        assert!(compset.size() == 1);

        Ok(())
    }

    #[test]
    fn create_empty() -> Result<()> {
        let compset= ComponentSet::<Comp1>::new();
        assert!(compset.size() == 0);
        Ok(())
    }
}
