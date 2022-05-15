#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;

fn main() {
  #[cfg(target_os = "macos")]
  let builder = builder.menu(menu::menu());

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}