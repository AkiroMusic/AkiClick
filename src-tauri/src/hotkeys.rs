use crate::clicker::ClickEngine;
use crate::clicker::ClickMode;
use crate::config::Config;
use crate::i18n::Lang;
use crate::AppState;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Convert Windows VK code to Tauri Code enum.
/// Keep in sync with `VK_CODES` in `src/vkCodes.ts` on the frontend.
fn vk_to_code(vk: u32) -> Option<Code> {
    match vk {
        // Function keys
        112 => Some(Code::F1),
        113 => Some(Code::F2),
        114 => Some(Code::F3),
        115 => Some(Code::F4),
        116 => Some(Code::F5),
        117 => Some(Code::F6),
        118 => Some(Code::F7),
        119 => Some(Code::F8),
        120 => Some(Code::F9),
        121 => Some(Code::F10),
        122 => Some(Code::F11),
        123 => Some(Code::F12),
        // Number keys
        48 => Some(Code::Digit0),
        49 => Some(Code::Digit1),
        50 => Some(Code::Digit2),
        51 => Some(Code::Digit3),
        52 => Some(Code::Digit4),
        53 => Some(Code::Digit5),
        54 => Some(Code::Digit6),
        55 => Some(Code::Digit7),
        56 => Some(Code::Digit8),
        57 => Some(Code::Digit9),
        // Letter keys
        65 => Some(Code::KeyA),
        66 => Some(Code::KeyB),
        67 => Some(Code::KeyC),
        68 => Some(Code::KeyD),
        69 => Some(Code::KeyE),
        70 => Some(Code::KeyF),
        71 => Some(Code::KeyG),
        72 => Some(Code::KeyH),
        73 => Some(Code::KeyI),
        74 => Some(Code::KeyJ),
        75 => Some(Code::KeyK),
        76 => Some(Code::KeyL),
        77 => Some(Code::KeyM),
        78 => Some(Code::KeyN),
        79 => Some(Code::KeyO),
        80 => Some(Code::KeyP),
        81 => Some(Code::KeyQ),
        82 => Some(Code::KeyR),
        83 => Some(Code::KeyS),
        84 => Some(Code::KeyT),
        85 => Some(Code::KeyU),
        86 => Some(Code::KeyV),
        87 => Some(Code::KeyW),
        88 => Some(Code::KeyX),
        89 => Some(Code::KeyY),
        90 => Some(Code::KeyZ),
        // Numpad
        96 => Some(Code::Numpad0),
        97 => Some(Code::Numpad1),
        98 => Some(Code::Numpad2),
        99 => Some(Code::Numpad3),
        100 => Some(Code::Numpad4),
        101 => Some(Code::Numpad5),
        102 => Some(Code::Numpad6),
        103 => Some(Code::Numpad7),
        104 => Some(Code::Numpad8),
        105 => Some(Code::Numpad9),
        _ => None,
    }
}

pub fn is_known_vk(vk: u32) -> bool {
    vk_to_code(vk).is_some()
}

pub fn register_hotkeys(app_handle: &AppHandle, config: &Config) {
    unregister_all(app_handle);

    let lang = crate::i18n::lang_of(&config.lang);

    // Start hotkey (legacy INI key: `left`)
    if let Some(code) = vk_to_code(config.start_vk()) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let key_name = shortcut.into_string();
        let result = app_handle.global_shortcut().on_shortcut(
            shortcut,
            move |app, _shortcut, event| {
                // The plugin fires on both press and release; act on press only.
                if event.state != ShortcutState::Pressed {
                    return;
                }
                let state = app.state::<AppState>();
                if !*state.listening.lock() {
                    return;
                }
                let config = state.config.lock().clone().unwrap_or_default();
                let engine = get_or_create_engine(&state);
                if engine
                    .start(
                        app.clone(),
                        ClickMode::from(config.mode),
                        config.interval_ms(),
                        config.click_count(),
                    )
                    .is_ok()
                {
                    app.emit("click-state-changed", serde_json::json!({ "isRunning": true })).ok();
                }
            },
        );
        if let Err(e) = result {
            report_registration_error(app_handle, lang, "start", &key_name, e.to_string());
        }
    } else {
        report_invalid_vk(app_handle, lang, "start", config.start_vk());
    }

    // Stop hotkey (legacy INI key: `right`)
    if let Some(code) = vk_to_code(config.stop_vk()) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let key_name = shortcut.into_string();
        let result = app_handle.global_shortcut().on_shortcut(
            shortcut,
            move |app, _shortcut, event| {
                if event.state != ShortcutState::Pressed {
                    return;
                }
                stop_engine(app);
                // The click worker emits "click-state-changed" when it exits.
            },
        );
        if let Err(e) = result {
            report_registration_error(app_handle, lang, "stop", &key_name, e.to_string());
        }
    } else {
        report_invalid_vk(app_handle, lang, "stop", config.stop_vk());
    }

    // Exit hotkey (legacy INI key: `stop`) — gated on listening like the others
    if let Some(code) = vk_to_code(config.exit_vk()) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let key_name = shortcut.into_string();
        let result = app_handle.global_shortcut().on_shortcut(
            shortcut,
            move |app, _shortcut, event| {
                if event.state != ShortcutState::Pressed {
                    return;
                }
                let state = app.state::<AppState>();
                if !*state.listening.lock() {
                    return;
                }
                stop_engine(app);
                app.exit(0);
            },
        );
        if let Err(e) = result {
            report_registration_error(app_handle, lang, "exit", &key_name, e.to_string());
        }
    } else {
        report_invalid_vk(app_handle, lang, "exit", config.exit_vk());
    }
}

pub fn unregister_all(app_handle: &AppHandle) {
    if let Err(e) = app_handle.global_shortcut().unregister_all() {
        tracing::warn!("failed to unregister hotkeys: {e}");
    }
}

/// Stop the click engine if a session is running. Safe to call when idle.
/// The click worker emits "click-state-changed" when it exits.
pub fn stop_engine(app_handle: &AppHandle) {
    let state = app_handle.state::<AppState>();
    let engine = state.click_engine.lock().clone();
    if let Some(engine) = engine {
        if let Err(e) = engine.stop() {
            if e != "Not running" {
                tracing::warn!("failed to stop click engine: {e}");
            }
        }
    }
}

fn get_or_create_engine(state: &tauri::State<'_, AppState>) -> ClickEngine {
    let mut guard = state.click_engine.lock();
    if guard.is_none() {
        *guard = Some(ClickEngine::new());
    }
    guard.as_ref().unwrap().clone()
}

fn report_registration_error(app_handle: &AppHandle, lang: Lang, action: &str, key: &str, err: String) {
    tracing::error!("hotkey registration failed: action={action} key={key} error={err}");
    app_handle
        .emit("hotkey-error", serde_json::json!({ "message": lang.hotkey_error(action, key) }))
        .ok();
}

fn report_invalid_vk(app_handle: &AppHandle, lang: Lang, action: &str, vk: u32) {
    tracing::error!("invalid hotkey VK code: action={action} vk={vk}");
    app_handle
        .emit("hotkey-error", serde_json::json!({ "message": lang.hotkey_invalid(action, vk) }))
        .ok();
}
