#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::app;

struct Port(u16);

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    tauri::async_runtime::spawn(app(port));
    tauri::Builder::default()
        .manage(Port(port))
        .invoke_handler(tauri::generate_handler![get_port]) // add functions you want to call from the frontend here
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); // custom error msg to be printed upon activating the app here
}

/// A command to get the unused port, instead of 3000. This func is called in frontend/src/main.rs
#[tauri::command]
fn get_port(port: tauri::State<Port>) -> Result<String, String> {
    Ok(format!("{}", port.0))
}
