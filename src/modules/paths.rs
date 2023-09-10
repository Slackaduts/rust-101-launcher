use crate::modules::game::*;
use std::collections::HashSet;
use std::path::Path;

/// Returns if a provided path is a valid install for a given Game.
/// # Arguments
/// * path (&str): Desired path to check, as a string slice.
/// * game (&Game): Desired game to check with, as a reference to a Game struct.
pub fn game_install_exists(path: &str, game: &Game) -> bool {
    let dir_path: String = path.to_owned() + game.name + "\\";
    let game_path: &str = &(dir_path + game.name + ".exe");
 
    return Path::new(game_path).exists();
}

/// Returns the install directory for the provided game.
/// # Arguments
/// * game (&Game): Game struct to base paths on.
/// * custom_path (&str): Custom path to use. Use an empty string if you do not want this, as this is meant to intake strings from a GUI.
pub fn get_game_path(game: &Game, custom_path: &str) -> Option<String> {
    // Custom path takes priority
    match game_install_exists(custom_path, game) {
        true => {
            return Some(custom_path.to_owned());
        }
        false => (),
    }

    // Check steam and regular install paths (TODO: See if EU has different install paths)
    let paths: &[&str; 2] = &[
        "C:\\ProgramData\\KingsIsle Entertainment\\",
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\",
    ];

    // See if we can find an install by modifying the default paths based on the game and checking if those paths exist.
    let mut game_path: String = String::new();
    for path in paths.iter() {
        if game_install_exists(path, game) {
            // Modify game path to include game-specific data and be properly formatted.
            game_path.push_str(path);
            game_path.push_str(game.name);
            game_path.push_str("\\");
            return Some(game_path);
        }
    }

    // If we cannot find an install, return None as the game path.
    return None;
}

pub fn fix_src_path(path: &str, symbol: &str) -> String {
    let exclusions: HashSet<&str> = HashSet::from(["windows", "macos", "steam"]);
    let mut split_path: Vec<&str> = path.split(symbol).collect();

    let path_part: &str = match split_path.get(0) {
        Some(substr) => substr,
        None => return path.to_owned(),
    };

    if exclusions.contains(path_part) {
        split_path.remove(0);
        return split_path.join(symbol);
    }

    return path.to_owned()
}
