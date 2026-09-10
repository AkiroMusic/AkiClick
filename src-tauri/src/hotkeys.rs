use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Code, Modifiers};
use crate::AppState;
use crate::clicker::ClickMode;

/// Convert Windows VK code to Tauri Code enum
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

pub fn register_hotkeys(app_handle: &AppHandle, config: &crate::config::Config) {
    unregister_all(app_handle);

    // Register start hotkey (e.g., F9)
    if let Some(code) = vk_to_code(config.left) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
            let state = _app.state::<AppState>();
            let listening = *state.listening.lock().unwrap();
            if listening {
                let config = state.config.lock().unwrap().clone().unwrap_or_default();
                let mode = ClickMode::from(config.mode);
                let interval_ms = config.freq;
                let count = config.clicktimes;
                
                let engine = {
                    let mut click_engine_guard = state.click_engine.lock().unwrap();
                    if click_engine_guard.is_none() {
                        *click_engine_guard = Some(crate::clicker::ClickEngine::new());
                    }
                    click_engine_guard.as_ref().unwrap().clone()
                };
                
                if engine.start(mode, interval_ms, count).is_ok() {
                    _app.emit("click-state-changed", serde_json::json!({ "isRunning": true })).ok();
                }
            }
        });
    }

    // Register stop hotkey (e.g., F10)
    if let Some(code) = vk_to_code(config.right) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
            let state = _app.state::<AppState>();
            let engine = {
                let click_engine_guard = state.click_engine.lock().unwrap();
                click_engine_guard.as_ref().cloned()
            };
            
            if let Some(engine) = engine {
                if engine.stop().is_ok() {
                    _app.emit("click-state-changed", serde_json::json!({ "isRunning": false })).ok();
                }
            }
        });
    }

    // Register exit hotkey (e.g., F11)
    if let Some(code) = vk_to_code(config.stop) {
        let shortcut = Shortcut::new(Some(Modifiers::empty()), code);
        let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
            _app.exit(0);
        });
    }
}

pub fn unregister_all(app_handle: &AppHandle) {
    let _ = app_handle.global_shortcut().unregister_all();
}