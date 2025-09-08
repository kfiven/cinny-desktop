// src/menu.rs
use tauri::menu::{
  MenuBuilder, SubmenuBuilder, PredefinedMenuItem, MenuItemBuilder,
};
use tauri::AppHandle;

pub fn build_menu(app: &AppHandle) -> tauri::menu::Menu {
  let submenu_app = SubmenuBuilder::new(app, "Cinny")
    .about(Some(Default::default()))
    .separator()
    .hide()
    .hide_others()
    .show_all()
    .separator()
    .quit()
    .build()
    .unwrap();

  let submenu_edit = SubmenuBuilder::new(app, "Edit")
    .undo()
    .redo()
    .separator()
    .cut()
    .copy()
    .paste()
    .select_all()
    .build()
    .unwrap();

  let submenu_view = SubmenuBuilder::new(app, "View")
    .enter_fullscreen()
    .build()
    .unwrap();

  let submenu_window = SubmenuBuilder::new(app, "Window")
    .minimize()
    .zoom()
    .build()
    .unwrap();

  MenuBuilder::new(app)
    .item(&submenu_app)
    .item(&submenu_edit)
    .item(&submenu_view)
    .item(&submenu_window)
    .build()
    .unwrap()
}
