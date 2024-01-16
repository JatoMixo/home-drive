// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_lister;
use file_lister::get_elements_in_path;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_elements_in_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
