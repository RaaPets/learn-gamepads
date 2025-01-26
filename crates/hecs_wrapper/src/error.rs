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

    /*
    impl From(hecs::NoSuchEntity) InputSystemError {
        fn from(src: hecs::NoSuchEntity) -> Self {

        }
    }
    */
}
