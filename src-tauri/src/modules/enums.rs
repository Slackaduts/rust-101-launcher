use std::fmt;

///Possible ports for KI game launching, depending on platform.
#[derive(Clone)]
pub enum Platform {
    WINDOWS = 12500,
    MACOS = 12600,
    STEAM = 12700,
}

impl Copy for Platform {}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u16)
    }
}