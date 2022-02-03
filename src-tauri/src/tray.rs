use tauri::{
  AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Wry,
};

pub fn setup() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  let tray = SystemTray::new().with_menu(tray_menu);

  return tray;
}

pub fn handler(_: &AppHandle<Wry>, event: &SystemTrayEvent) {
  match event {
    SystemTrayEvent::MenuItemClick { id, .. } => {
      match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "hide" => {
          // let window = app.get_window("main").unwrap();
          // window.hide().unwrap();
          println!("Hide!")
        }
        _ => {}
      }
    }
    _ => {
      println!("uncovered");
    }
  }
}
