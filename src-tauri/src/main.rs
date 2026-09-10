#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clicker;
mod commands;
mod config;
mod hotkeys;
mod tray;

use config::Config;
use tauri::Manager;

pub struct AppState {
    pub click_engine: std::sync::Mutex<Option<clicker::ClickEngine>>,
    pub config: std::sync::Mutex<Option<Config>>,
    pub listening: std::sync::Mutex<bool>,
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
        .manage(AppState {
            click_engine: std::sync::Mutex::new(None),
            config: std::sync::Mutex::new(None),
            listening: std::sync::Mutex::new(false),
        })
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
            
            // Update state with loaded config
            {
                let state = app.state::<AppState>();
                let mut config_guard = state.config.lock().unwrap();
                *config_guard = Some(config.clone());
            }

            // Register global hotkeys
            {
                let state = app.state::<AppState>();
                let config_guard = state.config.lock().unwrap();
                hotkeys::register_hotkeys(&app.handle().clone(), config_guard.as_ref().unwrap());
            }

            // Create system tray
            tray::create_tray(app.handle().clone())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::toggle_listening,
            commands::get_listening,
            commands::get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}