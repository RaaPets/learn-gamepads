use eyre::{Result, eyre};
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use std::collections::HashSet;
use std::rc::Rc;

use crate::entity::Entity;

//  //  //  //  //  //  //  //
#[derive(Debug)]
pub struct World {
    entities: HashSet<Rc<Entity>>,
}

impl World {
    pub fn new() -> Self {
        trace!(" + App::new()");

        Self {
            entities: HashSet::new(),
        }
    }

    pub fn remove(&mut self, entity: Rc<Entity>) -> Result<()> {
        if self.entities.remove(&entity) {
            return Ok(())
        }
        Err(eyre!("World.remove(entity): there is no the entity"))
    }

    pub fn create(&mut self) -> Rc<Entity> {
        let new_entity = Entity::new();
        self.entities.insert(new_entity.clone());
        new_entity
    }

    pub fn size(&self) -> usize {
        self.entities.len()
    }
}

impl Drop for World {
    fn drop(&mut self) {
        trace!(" - App::drop()");
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod base {
    use super::*;

    #[test]
    fn twice_remove_entity() -> Result<()> {
        let mut world = World::new();
        let ent2 = world.create();
        world.remove(ent2.clone())?;
        assert!(world.remove(ent2).is_err());
        Ok(())
    }

    #[test]
    fn remove_entity() -> Result<()> {
        let mut world = World::new();
        let ent1 = world.create();
        let ent2 = world.create();
        let ent3 = world.create();
        assert!(world.size() == 3);
        world.remove(ent2)?;
        assert!(world.size() == 2);
        world.remove(ent1)?;
        assert!(world.size() == 1);
        world.remove(ent3)?;
        assert!(world.size() == 0);
        Ok(())
    }

    #[test]
    fn create_3entity() -> Result<()> {
        let mut world = World::new();
        let _ent1 = world.create();
        let _ent2 = world.create();
        let _ent3 = world.create();
        assert!(world.size() == 3);
        Ok(())
    }

    #[test]
    fn create_1entity() -> Result<()> {
        let mut world = World::new();
        let _ent1 = world.create();
        assert!(world.size() == 1);
        Ok(())
    }

    #[test]
    fn create_empty() -> Result<()> {
        let world = World::new();
        assert!(world.size() == 0);
        Ok(())
    }
}
