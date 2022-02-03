#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu;
mod tray;

fn main() {
  tauri::Builder::default()
    .menu(menu::setup())
    .on_menu_event(|event| menu::handler(event))
    .system_tray(tray::setup())
    .on_system_tray_event(|app, event| tray::handler(&app, &event))
    .invoke_handler(tauri::generate_handler![my_custom_command, write_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() -> String {
  println!("Invoked from JavaScript");

  return "Hello from Rust!".into();
}

#[tauri::command]
fn write_file() -> String {
  println!("Write file");

  return "Hello from Rust!".into();
}
