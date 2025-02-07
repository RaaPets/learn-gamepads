use std::fmt;

//  //  //  //  //  //  //  //
pub mod input_system {

    use super::*;
    pub type Result = std::result::Result<(), InputSystemError>;

    #[derive(Debug, PartialEq, Eq)]
    pub enum InputSystemError {
        NoPlayerToSend,
    }

    impl std::error::Error for InputSystemError {}
    impl fmt::Display for InputSystemError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::NoPlayerToSend => f.write_str("no player to send input commands"),
            }
        }
    }
}

//  //  //  //  //  //  //  //
/*
pub mod pre_rendering_system {

    use super::*;
    pub type Result = std::result::Result<(), PreRenderingSystemError>;

    #[derive(Debug, PartialEq, Eq)]
    pub enum PreRenderingSystemError {
        NoPlayerForCentering,
        PlayerHasNoPosition,
    }

    impl std::error::Error for PreRenderingSystemError {}
    impl fmt::Display for PreRenderingSystemError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::NoPlayerForCentering => f.write_str("no player to centralizing positions"),
                Self::PlayerHasNoPosition => f.write_str("impossible centralizing due to player has no position"),
            }
        }
    }
}
*/
