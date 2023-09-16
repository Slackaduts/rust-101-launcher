// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use modules::webdriver::get_patch_urls;
use modules::game::*;
use modules::enums::Platform;
pub mod modules;





// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn print_patch_urls() {
    let game = PIRATE101;
    let platform = Platform::WINDOWS;
    let (file_list_url, base_url) = match get_patch_urls(&game, &platform).await {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    
    println!("{}", file_list_url);
    println!("{}", base_url);
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![print_patch_urls])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}
