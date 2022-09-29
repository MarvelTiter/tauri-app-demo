#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet2(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn update_title(title: &str, win: tauri::Window) {
    win.set_title(title).ok();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, greet2, update_title])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
