use std::path::Path;
use std::fs::read_to_string;

pub fn get_stored_revision(game_path: &str) -> Result<String, String> {
    let revision_path: &str = &(game_path.to_owned() + "Bin\\revision.dat");
    if !Path::new(revision_path).exists() {
        return Err(format!("Cannot find {}.", revision_path));
    }

    let contents: String = match read_to_string(revision_path) {
        Ok(fc) => fc,
        Err(e) => return Err(format!("Error occured when trying to read {}: {}", revision_path, e)),
    };

    let result: String = contents.replace("\n", "");
    return Ok(result);
}