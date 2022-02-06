#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::Local;
use notify_rust::Notification;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use tauri::Manager;
use tauri::WindowBuilder;

mod menu;
mod tray;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      let version = app.package_info().version.to_string();
      let mut title = "Tauri Demo v".to_string();
      title.push_str(&version);

      main_window.set_title(&title).unwrap();

      Ok(())
    })
    .create_window(
      "main".to_string(),
      tauri::WindowUrl::App("index.html".into()),
      move |window_builder, webview_attributes| (window_builder.menu(menu::setup()), webview_attributes),
    )
    .on_menu_event(|event| menu::handler(event))
    .system_tray(tray::setup())
    .on_system_tray_event(|app, event| tray::handler(&app, &event))
    .invoke_handler(tauri::generate_handler![
      close_splashscreen,
      hello_rust,
      read_file,
      write_file,
      notify,
      open_window,
      hide_broadcast_window
    ])
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


#[tauri::command]
fn open_window(window: tauri::Window) {
  if let Some(broadcast) = window.get_window("broadcast") {
    broadcast.show().unwrap();
  }
}

#[tauri::command]
fn hide_broadcast_window(window: tauri::Window) {

  println!("{:?}", "hide_broadcast_window");

  if let Some(broadcast) = window.get_window("broadcast") {
    broadcast.hide().unwrap();
  }

  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }

  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}
