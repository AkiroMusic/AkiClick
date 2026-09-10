use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Code, Modifiers};

pub fn register_default_hotkeys(app_handle: AppHandle, config: crate::config::Config) {
    register_hotkeys_internal(&app_handle, config.left, config.right, config.stop);
}

pub fn register_hotkeys(app_handle: AppHandle, left: u32, right: u32, stop: u32) {
    register_hotkeys_internal(&app_handle, left, right, stop);
}

fn register_hotkeys_internal(app_handle: &AppHandle, _start_vk: u32, _stop_vk: u32, _exit_vk: u32) {
    // Unregister all existing shortcuts first
    let _ = app_handle.global_shortcut().unregister_all();

    // Register start hotkey (F9 by default = VK 120)
    let shortcut = Shortcut::new(Some(Modifiers::empty()), Code::F9);
    let app_handle_clone = app_handle.clone();
    let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
        app_handle_clone.emit("start-click-hotkey", ()).ok();
    });

    // Register stop hotkey (F10 by default = VK 121)
    let shortcut = Shortcut::new(Some(Modifiers::empty()), Code::F10);
    let app_handle_clone = app_handle.clone();
    let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
        app_handle_clone.emit("stop-click-hotkey", ()).ok();
    });

    // Register exit hotkey (F11 by default = VK 122)
    let shortcut = Shortcut::new(Some(Modifiers::empty()), Code::F11);
    let app_handle_clone = app_handle.clone();
    let _ = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
        app_handle_clone.emit("exit-hotkey", ()).ok();
    });
}

pub fn unregister_all(app_handle: &AppHandle) {
    let _ = app_handle.global_shortcut().unregister_all();
}