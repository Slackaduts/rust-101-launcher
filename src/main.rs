use modules::webdriver::get_patch_urls;
use modules::game::*;
use modules::enums::Platform;
pub mod modules;


#[tokio::main]
async fn main() {
    let game = PIRATE101;
    let platform = Platform::WINDOWS;
    let (file_list_url, base_url) = match get_patch_urls(&game, &platform).await {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("{}", file_list_url);
    println!("{}", base_url);
}
