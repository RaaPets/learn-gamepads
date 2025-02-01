pub(super) fn generate_list(list: &[hecs::Entity]) -> Vec<(hecs::Entity, hecs::Entity)> {
    let n = list.len() as isize;
    let capa = n*(n-1)/2;
    let mut pairs = Vec::with_capacity(capa as usize);
    for primary in 0..(n - 1) {
        for secondary in (primary + 1)..list.len() as isize {
            let ent_a = list[primary as usize];
            let ent_b = list[secondary as usize];
            if ent_a != ent_b {
                pairs.push((ent_a, ent_b));
            }
        }
    }

    pairs
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod generate_list_test {
    use super::*;

    #[test]
    fn triple() {
        let mut world = hecs::World::new();
        let _a = world.spawn((false, 2));
        let _b = world.spawn((false, 2));
        let _c = world.spawn((false, 2));
        {
            let list = world
                .iter()
                .map(|ent_ref| ent_ref.entity())
                .collect::<Vec<_>>();
            let pairs = generate_list(&list);
            assert!(pairs.len() == 3);
        }
    }

    #[test]
    fn couple() {
        let mut world = hecs::World::new();
        let a = world.spawn((false, 2));
        let b = world.spawn((false, 2));
        {
            let list = [a, b];
            let pairs = generate_list(&list);
            assert!(pairs.len() == 1);
        }
        {
            let list = world
                .iter()
                .map(|ent_ref| ent_ref.entity())
                .collect::<Vec<_>>();
            let pairs = generate_list(&list);
            assert!(pairs.len() == 1);
        }
    }

    #[test]
    fn single() {
        let mut world = hecs::World::new();
        let a = world.spawn((false, 2));
        {
            let list = [a];
            let pairs = generate_list(&list);
            assert!(pairs.is_empty());
        }
        {
            let list = [a, a];
            let pairs = generate_list(&list);
            assert!(pairs.is_empty());
        }
        {
            let list = [a, a, a];
            let pairs = generate_list(&list);
            assert!(pairs.is_empty());
        }
    }

    #[test]
    fn empty() {
        let list: Vec<hecs::Entity> = vec![];
        let pairs = generate_list(&list);
        assert!(pairs.is_empty());
    }
}
