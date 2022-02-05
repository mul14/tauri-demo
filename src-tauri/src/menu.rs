use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn setup() -> Menu {

  #![allow(unused_variables)]
  let app_menu = Submenu::new(
    "",
    Menu::new()
      .add_native_item(MenuItem::About(String::from("Tauri Demo")))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Services)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::HideOthers)
      .add_native_item(MenuItem::ShowAll)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Quit),
  );

  let file_menu = Submenu::new(
    "File",
    Menu::new()
      .add_native_item(MenuItem::About("Demo App".to_string()))
      .add_item(CustomMenuItem::new("close".to_string(), "Close"))
      .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
  );

  let edit_menu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::SelectAll),
  );

  let window_menu = Submenu::new(
    "Window",
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Zoom)
      .add_native_item(MenuItem::EnterFullScreen),
  );

  let view_menu = Submenu::new(
    "View",
    Menu::new()
      .add_native_item(MenuItem::EnterFullScreen),
  );

  #[cfg(target_os = "macos")]
  let menu = Menu::new()
    .add_submenu(app_menu)
    .add_submenu(file_menu)
    .add_submenu(edit_menu)
    .add_submenu(view_menu)
    .add_submenu(window_menu)
    .add_item(CustomMenuItem::new("custom_menu", "Custom Menu"));

  #[cfg(not(target_os = "macos"))]
  let menu = Menu::new()
    .add_submenu(file_menu)
    .add_submenu(edit_menu)
    .add_submenu(view_menu)
    .add_submenu(window_menu)
    .add_item(CustomMenuItem::new("custom_menu", "Custom Menu"));

  return menu;
}

pub fn handler(event: tauri::WindowMenuEvent) {
  match event.menu_item_id() {
    "quit" => {
      std::process::exit(0);
    }
    "close" => {
      event.window().close().unwrap();
    }
    _ => {}
  }
}
