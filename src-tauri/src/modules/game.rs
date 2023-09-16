/**
Defines game-specific window/executable data.
*/
pub struct Game<'a> {
    pub name: &'a str,
    pub window_class: &'a str,
    pub executable: &'a str,
}

/**
Wizard101-specific data.
*/
pub const WIZARD101: Game = Game {
    name: "Wizard101",
    window_class: "Wizard Graphical Client",
    executable: "WizardGraphicalClient",
};

/**
Pirate101-specific data.
*/
pub const PIRATE101: Game = Game {
    name: "Pirate101",
    window_class: "Client",
    executable: "Pirate",
};
