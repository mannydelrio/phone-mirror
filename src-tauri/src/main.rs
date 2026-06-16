#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adb;
mod video;
mod state;
mod server;

use state::AppState;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Device commands
            adb::device::list_devices,
            adb::device::connect_wifi,
            adb::device::disconnect_device,
            // Input commands
            adb::input::tap,
            adb::input::swipe,
            adb::input::key,
            adb::input::text,
            adb::input::home,
            adb::input::back,
            adb::input::recent_apps,
            adb::input::get_display_info,
            // Video commands
            video::start_mirroring,
            video::stop_mirroring,
            video::start_recording,
            video::stop_recording,
            // Server commands
            server::start_web_server,
            server::stop_web_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
