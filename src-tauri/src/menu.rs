use tauri::{menu::{Menu, Submenu, MenuItem}, about, quit, Manager};

pub fn build_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Menu<R> {
    let app_menu = Submenu::new(
        "App",
        Menu::new()
            .add(about(app, "Cinny", "A matrix app"))
            .add(MenuItem::separator())
            .add(quit(app, "Quit", Some("CmdOrCtrl+Q")))
    );

    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add(MenuItem::undo(app, "Undo", None))
            .add(MenuItem::redo(app, "Redo", None))
            .add(MenuItem::separator())
            .add(MenuItem::cut(app, "Cut", None))
            .add(MenuItem::copy(app, "Copy", None))
            .add(MenuItem::paste(app, "Paste", None))
            .add(MenuItem::select_all(app, "Select All", None))
    );

    let view_menu = Submenu::new(
        "View",
        Menu::new()
            .add(MenuItem::enter_fullscreen(app, "Fullscreen", None))
    );

    let window_menu = Submenu::new(
        "Window",
        Menu::new()
            .add(MenuItem::minimize(app, "Minimize", None))
            .add(MenuItem::zoom(app, "Zoom", None))
    );

    Menu::new()
        .add_submenu(app_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(window_menu)
}
