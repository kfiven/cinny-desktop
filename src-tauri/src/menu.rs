use tauri::{
    menu::{Menu, Submenu, MenuItem, NativeItem},
    Manager, AppHandle, Runtime,
};

pub(crate) fn build_menu<R: Runtime>(app: &AppHandle<R>) -> Menu<R> {
    let about_metadata = tauri::menu::AboutMetadata::default();

    let app_menu = Submenu::new(
        app,
        "Cinny",
        Menu::with_items(
            app,
            &[
                MenuItem::Native(NativeItem::About("Cinny".into(), about_metadata)),
                MenuItem::Native(NativeItem::Separator),
                MenuItem::Native(NativeItem::Hide),
                MenuItem::Native(NativeItem::HideOthers),
                MenuItem::Native(NativeItem::ShowAll),
                MenuItem::Native(NativeItem::Separator),
                MenuItem::Native(NativeItem::Quit),
            ],
        ),
        true,
    );

    let edit_menu = Submenu::new(
        app,
        "Edit",
        Menu::with_items(
            app,
            &[
                MenuItem::Native(NativeItem::Undo),
                MenuItem::Native(NativeItem::Redo),
                MenuItem::Native(NativeItem::Separator),
                MenuItem::Native(NativeItem::Cut),
                MenuItem::Native(NativeItem::Copy),
                MenuItem::Native(NativeItem::Paste),
                MenuItem::Native(NativeItem::SelectAll),
            ],
        ),
        true,
    );

    let view_menu = Submenu::new(
        app,
        "View",
        Menu::with_items(app, &[MenuItem::Native(NativeItem::EnterFullScreen)]),
        true,
    );

    let window_menu = Submenu::new(
        app,
        "Window",
        Menu::with_items(
            app,
            &[
                MenuItem::Native(NativeItem::Minimize),
                MenuItem::Native(NativeItem::Zoom),
            ],
        ),
        true,
    );

    Menu::with_items(app, &[
        MenuItem::Submenu(app_menu),
        MenuItem::Submenu(edit_menu),
        MenuItem::Submenu(view_menu),
        MenuItem::Submenu(window_menu),
    ])
}
