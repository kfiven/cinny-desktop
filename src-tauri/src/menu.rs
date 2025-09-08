use tauri::{
    menu::{
        AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
    },
    Menu,
};

pub fn menu() -> Menu {
    MenuBuilder::new()
        .items([
            SubmenuBuilder::new(
                "Cinny",
                MenuBuilder::new()
                    .items([
                        PredefinedMenuItem::about("Cinny", AboutMetadataBuilder::new().build()).into(),
                        PredefinedMenuItem::separator().into(),
                        PredefinedMenuItem::hide().into(),
                        PredefinedMenuItem::hide_others().into(),
                        PredefinedMenuItem::show_all().into(),
                        PredefinedMenuItem::separator().into(),
                        PredefinedMenuItem::quit().into(),
                    ])
                    .build(),
            )
            .build()
            .into(),

            SubmenuBuilder::new(
                "Edit",
                MenuBuilder::new()
                    .items([
                        PredefinedMenuItem::undo().into(),
                        PredefinedMenuItem::redo().into(),
                        PredefinedMenuItem::separator().into(),
                        PredefinedMenuItem::cut().into(),
                        PredefinedMenuItem::copy().into(),
                        PredefinedMenuItem::paste().into(),
                        PredefinedMenuItem::select_all().into(),
                    ])
                    .build(),
            )
            .build()
            .into(),

            SubmenuBuilder::new(
                "View",
                MenuBuilder::new()
                    .items([
                        PredefinedMenuItem::enter_fullscreen().into(),
                    ])
                    .build(),
            )
            .build()
            .into(),

            SubmenuBuilder::new(
                "Window",
                MenuBuilder::new()
                    .items([
                        PredefinedMenuItem::minimize().into(),
                        PredefinedMenuItem::zoom().into(),
                    ])
                    .build(),
            )
            .build()
            .into(),
        ])
}
