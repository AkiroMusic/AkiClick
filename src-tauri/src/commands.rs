use tauri::{AppHandle, State, Emitter};
use crate::AppState;
use crate::clicker::{ClickEngine, ClickMode};
use crate::config::Config;

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone().unwrap_or_default())
}

#[tauri::command]
pub async fn save_config(config: Config, state: State<'_, AppState>, app_handle: AppHandle) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())?;
    *state.config.lock().unwrap() = Some(config.clone());
    app_handle.emit("config-changed", config).ok();
    Ok(())
}

#[tauri::command]
pub async fn start_clicking(mode: u32, interval_ms: u32, count: u32, state: State<'_, AppState>, app_handle: AppHandle) -> Result<(), String> {
    let engine = {
        let mut click_engine_guard = state.click_engine.lock().unwrap();
        if click_engine_guard.is_none() {
            *click_engine_guard = Some(ClickEngine::new());
        }
        click_engine_guard.as_ref().unwrap().clone()
    };

    let click_mode = ClickMode::from(mode);
    engine.start(click_mode, interval_ms, count).map_err(|e| e.to_string())?;

    app_handle.emit("click-state-changed", serde_json::json!({ "isRunning": true })).ok();
    Ok(())
}

#[tauri::command]
pub async fn stop_clicking(state: State<'_, AppState>, app_handle: AppHandle) -> Result<(), String> {
    let engine = {
        let click_engine_guard = state.click_engine.lock().unwrap();
        click_engine_guard.as_ref().cloned()
    };

    if let Some(engine) = engine {
        engine.stop().map_err(|e| e.to_string())?;
    }

    app_handle.emit("click-state-changed", serde_json::json!({ "isRunning": false })).ok();
    Ok(())
}

#[tauri::command]
pub async fn get_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[tauri::command]
pub async fn register_hotkeys(left: u32, right: u32, stop: u32, app_handle: AppHandle) -> Result<(), String> {
    crate::hotkeys::register_hotkeys(app_handle, left, right, stop);
    Ok(())
}