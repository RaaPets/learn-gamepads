use super::*;
use player::GameInputCommand;

//  //  //  //  //  //  //  //
pub(crate) fn clear(players: hecs::QueryMut<&mut PlayerInput>) {
    for (_id, input) in players {
        input.clear();
    }
}

pub(crate) fn update(players: hecs::QueryMut<(&mut Position, &PlayerInput)>) -> Option<Position> {
    let mut first_player_position = None;

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
                    last_ch = Some(ch);
                    todo!("TypeDigital({})", ch);
                }
            }
        }
        *position += Position::from_tuple((di, dj));
        if first_player_position.is_none() {
            first_player_position = Some(*position);
        }
    }
    first_player_position
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod player_input_test {
    use super::*;

    fn invoke_update(world: &mut hecs::World) -> Option<Position> {
        update(world.query_mut::<(&mut Position, &player::PlayerInput)>())
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
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let p = invoke_update(&mut world);
            let pos2 = Position::from((2., 4.));
            assert!(p == Some(pos2));
        }
    }

    #[test]
    fn once_move_player() {
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
            p_input.add_last(GameInputCommand::OnceUp);
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let p = invoke_update(&mut world);
            let pos2 = Position::from((2., 4.));
            assert!(p == Some(pos2));
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::OnceDown);
            p_input.add_last(GameInputCommand::OnceLeft);
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let p = invoke_update(&mut world);
            let pos2 = Position::from((1., 6.));
            assert!(p == Some(pos2));
        }
        {
            let mut p_input = PlayerInput::new();
            p_input.add_last(GameInputCommand::OnceRight);
            p_input.add_last(GameInputCommand::OnceUp);
            let pos = Position::from((2., 5.));
            let _ = world.insert_one(player, p_input);
            let _ = world.insert_one(player, pos);
            //
            let p = invoke_update(&mut world);
            let pos2 = Position::from((3., 4.));
            assert!(p == Some(pos2));
        }
    }
}
