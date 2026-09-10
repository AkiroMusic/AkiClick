use tauri::{AppHandle, State, Emitter};
use crate::AppState;
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
pub async fn toggle_listening(state: State<'_, AppState>, app_handle: AppHandle) -> Result<bool, String> {
    let mut listening = state.listening.lock().unwrap();
    *listening = !*listening;
    let new_state = *listening;
    drop(listening);
    
    app_handle.emit("listening-changed", new_state).ok();
    Ok(new_state)
}

#[tauri::command]
pub async fn get_listening(state: State<'_, AppState>) -> Result<bool, String> {
    let listening = state.listening.lock().unwrap();
    Ok(*listening)
}

#[tauri::command]
pub async fn get_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}