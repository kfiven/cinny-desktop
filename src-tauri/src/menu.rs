use tauri::menu::*;

fn menu() -> Menu {
tauri::Builder::default()
    .setup(|app| {
        let handle = app.handle();
        let about = MenuBuilder::new(app)
            .about(app)
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;
        app.set_menu(about);

        let edit = MenuBuilder::new(app)
            .undo()
            .redo()
            .separator()
            .cut()
            .copy()
            .paste()
            .select_all()
            .build()?;
        app.set_menu(edit);

        let view = MenuBuilder::new(app)
            .fullscreen()
            .build()?;
        app.set_menu(view);

        let window = MenuBuilder::new(app)
            .minimize()
            .build()?;
        app.set_menu(window);
        Ok(())
    })
}
