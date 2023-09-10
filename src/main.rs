use crate::modules::patching::get_stored_revision;
use crate::modules::paths::get_game_path;
use crate::modules::game::*;
pub mod modules;


fn main() {
    let game: &Game = &(PIRATE101);
    let game_path: String = match get_game_path(game, "") {
        Some(path) => path,
        None => panic!(),
    };

    
    match get_stored_revision(&(game_path)) {
        Ok(revision) => println!("{}", revision),
        Err(e) => eprintln!("{}", e),
    }
}
