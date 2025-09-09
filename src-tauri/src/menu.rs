#[cfg(target_os = "macos")]
use tauri::{
    menu::{MenuBuilder, PredefinedMenuItem},
    App,
};

#[cfg(target_os = "macos")]
pub fn create_menu(app: &App) -> tauri::Result<tauri::menu::Menu> {
    Ok(MenuBuilder::new(app)
        .app_menu(|menu| {
            menu
                .about(None)
                .separator()
                .hide
                .hide_others
                .show_all
                .separator()
                .quit
        })
        .submenu("Edit", |menu| {
            menu
                .undo
                .redo
                .separator()
                .cut
                .copy
                .paste
                .select_all
        })
        .submenu("View", |menu| {
            menu
                .enter_fullscreen
        })
        .submenu("Window", |menu| {
            menu
                .minimize
                .zoom
        })
        .build()?)
}
