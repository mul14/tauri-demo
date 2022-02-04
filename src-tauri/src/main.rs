#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::Local;
use notify_rust::Notification;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

mod menu;
mod tray;

fn main() {
  tauri::Builder::default()
    .menu(menu::setup())
    .on_menu_event(|event| menu::handler(event))
    .system_tray(tray::setup())
    .on_system_tray_event(|app, event| tray::handler(&app, &event))
    .invoke_handler(tauri::generate_handler![hello_rust, read_file, write_file, notify])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello_rust() -> String {
  println!("Invoked from JavaScript");

  return "Hello from Rust".into();
}

#[tauri::command]
fn write_file(path: String) -> String {
  let target_path = Path::new(&path);

  let unix_timestamp = Local::now();

  println!("{:?}", unix_timestamp.naive_local().timestamp());

  let mut f = OpenOptions::new()
    .append(true)
    .create(true)
    .open(target_path)
    .expect("Unable to write file");

  let data = unix_timestamp.to_rfc2822() + "\n";

  f.write_all(data.as_bytes()).expect("Unable to write data");

  return data.to_string();
}

#[tauri::command]
fn read_file(path: String) -> String {
  let target_path = Path::new(&path);

  let result = read_to_string(target_path).unwrap_or_default();

  println!("{}", result.to_string());

  return result;
}

#[tauri::command]
fn notify(title: String, body: String) {
  let result = Notification::new()
    .summary(title.as_str())
    .body(body.as_str())
    .show();

  println!("{:?}", result);
}
