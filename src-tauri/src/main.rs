#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clicker;
mod commands;
mod config;
mod hotkeys;
mod i18n;
mod tray;

use config::Config;
use parking_lot::Mutex;
use tauri::{Manager, RunEvent};

pub struct AppState {
    pub click_engine: Mutex<Option<clicker::ClickEngine>>,
    pub config: Mutex<Option<Config>>,
    pub listening: Mutex<bool>,
    pub tray_menu: Mutex<Option<tray::TrayMenu>>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            click_engine: Mutex::new(None),
            config: Mutex::new(None),
            listening: Mutex::new(false),
            tray_menu: Mutex::new(None),
        })
        .setup(|app| {
            // Load config; a corrupt/partial file falls back to defaults with
            // a warning instead of failing silently.
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    tracing::warn!("failed to load config, using defaults: {e}");
                    Config::default()
                }
            };

            // Write the default config on first run
            if let Ok(path) = Config::config_path() {
                if !path.exists() {
                    if let Err(e) = config.save() {
                        tracing::warn!("failed to write default config: {e}");
                    }
                }
            }

            // Apply the stored always-on-top preference; the tauri.conf.json
            // window default is always-on-top, the user may have turned it off.
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(config.always_on_top_enabled());
            }

            *app.state::<AppState>().config.lock() = Some(config.clone());

            // Register global hotkeys
            hotkeys::register_hotkeys(&app.handle().clone(), &config);

            // Create system tray
            tray::create_tray(app.handle().clone())?;

            tracing::info!("AkiClick v{} started", env!("CARGO_PKG_VERSION"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::toggle_listening,
            commands::save_preferences,
            commands::get_version,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let RunEvent::ExitRequested { .. } = event {
                // Release the click thread so a pending SendInput pair can't
                // leave the synthetic mouse button stuck down.
                hotkeys::stop_engine(app_handle);
            }
        });
}
