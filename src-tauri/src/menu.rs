use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, PredefinedMenuItem, SubmenuBuilder, MenuItemBuilder, CheckMenuItemBuilder};

pub(crate) fn menu() -> impl FnOnce(&AppHandle) -> Result<(), tauri::Error> {
    move |app| {
        // Create the "Cinny" submenu
        let cinny_submenu = SubmenuBuilder::new(app, "Cinny")
            .item(&PredefinedMenuItem::about("Cinny", app)?)
            .separator()
            .item(&PredefinedMenuItem::hide(app)?)
            .item(&PredefinedMenuItem::hide_others(app)?)
            .item(&PredefinedMenuItem::show_all(app)?)
            .separator()
            .item(&PredefinedMenuItem::quit(app)?)
            .build()?;

        // Create the "Edit" submenu
        let edit_submenu = SubmenuBuilder::new(app, "Edit")
            .item(&PredefinedMenuItem::undo(app)?)
            .item(&PredefinedMenuItem::redo(app)?)
            .separator()
            .item(&PredefinedMenuItem::cut(app)?)
            .item(&PredefinedMenuItem::copy(app)?)
            .item(&PredefinedMenuItem::paste(app)?)
            .item(&PredefinedMenuItem::select_all(app)?)
            .build()?;

        // Create the "View" submenu
        let view_submenu = SubmenuBuilder::new(app, "View")
            .item(MenuItemBuilder::with_id("enter_full_screen", "Enter Full Screen").build(app)?)
            .build()?;

        // Create the "Window" submenu
        let window_submenu = SubmenuBuilder::new(app, "Window")
            .item(MenuItemBuilder::with_id("minimize", "Minimize").build(app)?)
            .item(MenuItemBuilder::with_id("zoom", "Zoom").build(app)?)
            .build()?;

        // Build the main menu
        let menu = MenuBuilder::new(app)
            .item(&cinny_submenu)
            .item(&edit_submenu)
            .item(&view_submenu)
            .item(&window_submenu)
            .build()?;

        app.set_menu(menu)?;

        // Handle menu events
        app.on_menu_event(move |app, event| {
            match event.id() {
                "enter_full_screen" => {
                    println!("Enter Full Screen triggered!");
                }
                "minimize" => {
                    println!("Minimize triggered!");
                }
                "zoom" => {
                    println!("Zoom triggered!");
                }
                _ => {}
            }
        });

        Ok(())
    }
}
