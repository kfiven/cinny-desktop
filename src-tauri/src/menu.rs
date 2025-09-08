use tauri::menu::{Menu, MenuItem, Submenu, MenuBuilder};
use tauri::{AppHandle, Manager};

pub fn build_menu(app: &AppHandle) -> Menu {
    let app_menu = Submenu::new(
        app,
        "App",
        Menu::new(app)
            .unwrap()
            .append(MenuItem::new(app, "About", true).unwrap())
            .append(MenuItem::separator(app).unwrap())
            .append(MenuItem::new(app, "Quit", true).unwrap().with_id("quit").with_accelerator("CmdOrCtrl+Q")),
        false,
    );

    let edit_menu = Submenu::new(
        app,
        "Edit",
        Menu::new(app)
            .unwrap()
            .append(MenuItem::new(app, "Undo", true).unwrap().with_id("undo"))
            .append(MenuItem::new(app, "Redo", true).unwrap().with_id("redo"))
            .append(MenuItem::separator(app).unwrap())
            .append(MenuItem::new(app, "Cut", true).unwrap().with_id("cut"))
            .append(MenuItem::new(app, "Copy", true).unwrap().with_id("copy"))
            .append(MenuItem::new(app, "Paste", true).unwrap().with_id("paste"))
            .append(MenuItem::new(app, "Select All", true).unwrap().with_id("select_all")),
        false,
    );

    let view_menu = Submenu::new(
        app,
        "View",
        Menu::new(app)
            .unwrap()
            .append(MenuItem::new(app, "Enter Fullscreen", true).unwrap().with_id("fullscreen")),
        false,
    );

    let window_menu = Submenu::new(
        app,
        "Window",
        Menu::new(app)
            .unwrap()
            .append(MenuItem::new(app, "Minimize", true).unwrap().with_id("minimize")),
        false,
    );

    Menu::new(app)
        .unwrap()
        .append_submenu(app_menu)
        .append_submenu(edit_menu)
        .append_submenu(view_menu)
        .append_submenu(window_menu)
}
