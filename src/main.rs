use crate::modules::paths::{ game_install_exists, get_game_path };
use crate::modules::game::*;
pub mod modules;


fn main() {
    let path: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\";
    let game: &Game = &(PIRATE101);
    match game_install_exists(path, game) {
        true => println!("True"),
        false => println!("False"),
    }

    let game_path = match get_game_path(game, "") {
        Some(p) => p,
        None => String::new(),
    };

    println!("{}", game_path);
}
