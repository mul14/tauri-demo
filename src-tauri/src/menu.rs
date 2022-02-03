use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn setup() -> Menu {
  let separator = MenuItem::Separator;

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let about = MenuItem::About("Demo App".to_string());
  let file_menu = Submenu::new(
    "File",
    Menu::new()
      .add_item(quit)
      .add_item(close)
      .add_native_item(about),
  );

  let undo = MenuItem::Undo;
  let redo = MenuItem::Redo;
  let cut = MenuItem::Cut;
  let copy = MenuItem::Copy;
  let paste = MenuItem::Paste;
  let select_all = MenuItem::SelectAll;
  let edit_menu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(undo)
      .add_native_item(redo)
      .add_native_item(separator.clone())
      .add_native_item(cut)
      .add_native_item(copy)
      .add_native_item(paste)
      .add_native_item(separator.clone())
      .add_native_item(select_all),
  );

  let menu = Menu::new()
    .add_item(CustomMenuItem::new("somthing", "Something"))
    .add_submenu(file_menu)
    .add_submenu(edit_menu);

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
