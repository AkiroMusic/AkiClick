#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clicker;
mod commands;
mod config;
mod hotkeys;
mod tray;

use clicker::ClickEngine;
use config::Config;
use hotkeys::register_default_hotkeys;
use tray::create_tray;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    click_engine: std::sync::Mutex<Option<ClickEngine>>,
    config: std::sync::Mutex<Option<Config>>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::default())
        .setup(|app| {
            // Load config
            let config = Config::load().unwrap_or_default();
            
            // Save default config if it doesn't exist (first run)
            let config_path = Config::config_path().ok();
            if let Some(path) = config_path {
                if !path.exists() {
                    let _ = config.save();
                }
            }
            
            *app.state::<AppState>().config.lock().unwrap() = Some(config.clone());

            // Register global hotkeys
            register_default_hotkeys(app.handle().clone(), config.clone());

            // Create system tray
            create_tray(app.handle().clone())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::start_clicking,
            commands::stop_clicking,
            commands::get_version,
            commands::register_hotkeys,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}