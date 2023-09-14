use std::fmt;

///Possible ports for KI game launching, depending on platform.
pub enum Platform {
    WINDOWS = 12500,
    MACOS = 12600,
    STEAM = 12700,
}

//Implement display for Platform enum such that it prints the port number.
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::WINDOWS => write!(f, "{}", 12500),
            Platform::MACOS => write!(f, "{}", 12600),
            Platform::STEAM => write!(f, "{}", 12700),
        }
    }
}