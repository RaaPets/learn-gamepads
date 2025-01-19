//  //  //  //  //  //  //  //
pub struct GamepadHandler {
    gamepads: gamepads::Gamepads,
}

impl GamepadHandler {
    pub fn new() -> Self {
        Self {
            gamepads: gamepads::Gamepads::new(),
        }
    }

    pub fn update(&mut self) {
        self.gamepads.poll();
    }

    pub fn get_gamepad(&self, index: u8) -> Option<gamepads::Gamepad> {
        for gamepad in self.gamepads.all() {
            if gamepad.id().value() == index {
                return Some(gamepad);
            }
        }

        None
    }
}
