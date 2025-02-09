use super::*;
use crate::error::input_system::*;

//  //  //  //  //  //  //  //
impl super::RaaWorld {
    pub fn send_to_player(&mut self, inputs: Vec<GameInputCommand>) -> Result {
        for (_id, player_input) in self.world.query_mut::<&mut player::PlayerInput>() {
            player_input.clear();
            for cmd in inputs.into_iter() {
                player_input.add_last(cmd);
            }
            return Ok(());
        }
        Err(InputSystemError::NoPlayerToSend)
    }
}
