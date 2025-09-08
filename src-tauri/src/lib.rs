#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod menu; // Ensure this is present if building on macOS

use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

pub fn run() {
    let port: u16 = 44548;
    let context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(menu::menu());
    }

    builder
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(move |app| {
            let url = format!("http://localhost:{}", port).parse().unwrap();
            let window_url = WebviewUrl::External(url);
            WebviewWindowBuilder::new(app, "main".to_string(), window_url)
                .title("Cinny")
                .build()?;
            Ok(())
        })
        .run(context)
        .expect("error while building tauri application");
}
