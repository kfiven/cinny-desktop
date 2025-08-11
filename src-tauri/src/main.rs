#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;

use tauri::{AppBuilder, AppContext, AppHandle, Manager, Runtime, WindowUrl, AppUrl};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 44548;
    let url = format!("http://localhost:{}", port);

    // Create Tauri app builder
    let mut builder = tauri::AppBuilder::new()
        .setup(|app: &AppHandle<_>| {
            let window_url = WindowUrl::External(url.parse().unwrap());
            let mut config = app.config_mut();
            config.build.dist_dir = AppUrl::Url(window_url.clone());
            config.build.dev_path = AppUrl::Url(window_url.clone());
            Ok(())
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(menu::menu());
    }

    // Register plugins (ensure plugins are v2 compatible)
    builder = builder
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_window_state::Builder::default().build());

    builder
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("error while building tauri application: {e}"))
}
