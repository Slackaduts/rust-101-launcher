use modules::webdriver::get_patch_urls;
pub mod modules;


#[tokio::main]
async fn main() {
    let patch_urls = match get_patch_urls().await {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("{}", patch_urls);
}
