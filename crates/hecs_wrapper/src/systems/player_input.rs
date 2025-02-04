use super::*;
use player::GameInputCommand;

//  //  //  //  //  //  //  //
pub(crate) fn update(players: hecs::QueryMut<(&mut Position, &mut PlayerInput)>) -> Option<char> {
    let mut res_char = None;

    for (_id, (position, input)) in players {
        let mut di = 0.;
        let mut dj = 0.;
        let mut last_ch = None;
        for cmd in input.input_buffer.iter() {
            match cmd {
                GameInputCommand::OnceUp => dj -= 1.,
                GameInputCommand::OnceDown => dj += 1.,
                GameInputCommand::OnceLeft => di -= 1.,
                GameInputCommand::OnceRight => di += 1.,
                GameInputCommand::Accelerate((ddi, ddj)) => {
                    di += (*ddi as f64) / 3.5;
                    dj -= (*ddj as f64) / 3.5;
                }
                GameInputCommand::TypeDigital(ch) => {
                    last_ch = Some(*ch);
                }
            }
        }
        *position += Position::new(di, dj);
        if res_char.is_none() {
            res_char = last_ch;
        }
        input.clear();
    }
    res_char
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod player_input_test {
    use super::*;

    fn invoke_update(world: &mut hecs::World) -> Option<char> {
        update(world.query_mut::<(&mut Position, &mut player::PlayerInput)>())
    }

    #[test]
    fn type_digital_press() {
        let mut world = hecs::World::new();
        {
            let p = invoke_update(&mut world);
            assert!(p == None);
        }

        let player = world.spawn((false, 1));
        {
            let p = invoke_update(&mut world);
            assert!(p == None);
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::TypeDigital('1'));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, Position::default());
            //
            let inp = invoke_update(&mut world);
            assert!(inp == Some('1'));
        }
    }

    #[test]
    fn once_move_player() ->  eyre::Result<()> {
        let mut world = hecs::World::new();
        {
            let p = invoke_update(&mut world);
            assert!(p == None);
        }

        let player = world.spawn((false, 1));
        {
            let p = world.get::<&Position>(player);
            assert!(p.is_err());
        }
        {
            let _ = invoke_update(&mut world);
            let p = crate::world_wrapper::central_position::get_from_first(
                world.query::<(&CentralEntity, &CellPosition)>()
            );
            assert!(p == None);
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::OnceUp);
            let pos = Position::new(2., 5.);
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let _ = invoke_update(&mut world);
            let p = world.get::<&Position>(player)?;
            assert!(*p == Position::new(2., 4.));
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::OnceDown);
            p_input.add_last(GameInputCommand::OnceLeft);
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let _ = invoke_update(&mut world);
            let p = world.get::<&Position>(player)?;
            assert!(*p == Position::new(1., 6.));
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::OnceRight);
            p_input.add_last(GameInputCommand::OnceUp);
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let _ = invoke_update(&mut world);
            let p = world.get::<&Position>(player)?;
            assert!(*p == Position::new(3., 4.));
        }

        Ok(())
    }
}
