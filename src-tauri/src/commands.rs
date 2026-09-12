use crate::config::Config;
use crate::hotkeys;
use crate::tray;
use crate::AppState;
use tauri::{AppHandle, Emitter, Manager, State};

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    Ok(state.config.lock().clone().unwrap_or_default())
}

#[tauri::command]
pub async fn save_config(
    mut config: Config,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<Config, String> {
    config.sanitize();
    config.save().map_err(|e| {
        tracing::error!("failed to save config: {e}");
        e.to_string()
    })?;
    *state.config.lock() = Some(config.clone());

    // Re-register so hotkey changes take effect without a restart. Failures
    // (e.g. key already taken) are reported to the UI via "hotkey-error".
    hotkeys::register_hotkeys(&app_handle, &config);

    app_handle.emit("config-changed", &config).ok();
    Ok(config)
}

/// Shared by the IPC command and the tray menu item.
pub fn toggle_listening_state(app_handle: &AppHandle) -> bool {
    let state = app_handle.state::<AppState>();
    let new_state = {
        let mut listening = state.listening.lock();
        *listening = !*listening;
        *listening
    };
    // Turning listening off also stops an in-progress click session; the
    // worker emits "click-state-changed" when it exits.
    if !new_state {
        hotkeys::stop_engine(app_handle);
    }
    tray::refresh_menu(app_handle);
    new_state
}

#[tauri::command]
pub async fn toggle_listening(app_handle: AppHandle) -> Result<bool, String> {
    let new_state = toggle_listening_state(&app_handle);
    app_handle.emit("listening-changed", new_state).ok();
    Ok(new_state)
}

/// Persist UI preferences (theme/lang/always-on-top) without touching the
/// click settings the user may still be editing as an unsaved draft —
/// unlike save_config, this deliberately does not emit "config-changed".
#[tauri::command]
pub async fn save_preferences(
    theme: String,
    lang: String,
    always_on_top: u32,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let mut config = state.config.lock().clone().unwrap_or_default();
    let lang_changed = config.lang != lang;
    config.theme = theme;
    config.lang = lang;
    config.always_on_top = always_on_top;
    config.sanitize();
    config.save().map_err(|e| {
        tracing::error!("failed to save preferences: {e}");
        e.to_string()
    })?;
    *state.config.lock() = Some(config.clone());

    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.set_always_on_top(config.always_on_top_enabled());
    }
    if lang_changed {
        tray::refresh_menu(&app_handle);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
