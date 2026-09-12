use crate::i18n::{lang_of, Lang};
use crate::AppState;
use tauri::menu::{MenuBuilder, MenuItem, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{image::Image, AppHandle, Emitter, Manager, Wry};

/// Handles to the tray menu items so their labels can be updated when the
/// language or the listening state changes. Stored in AppState.
pub struct TrayMenu {
    pub show: MenuItem<Wry>,
    pub toggle_listening: MenuItem<Wry>,
    pub quit: MenuItem<Wry>,
}

/// Load icon embedded at compile time from icons/icon.png
fn load_icon() -> Image<'static> {
    let bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(bytes).expect("Failed to load embedded icon");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.into_raw();
    Image::new_owned(pixels, width, height)
}

fn listening_label(lang: Lang, listening: bool) -> &'static str {
    if listening {
        lang.disable_listening()
    } else {
        lang.enable_listening()
    }
}

/// Update tray menu labels from the current config language + listening state.
pub fn refresh_menu(app_handle: &AppHandle) {
    let state = app_handle.state::<AppState>();
    let config = state.config.lock().clone().unwrap_or_default();
    let lang = lang_of(&config.lang);
    let listening = *state.listening.lock();
    let guard = state.tray_menu.lock();
    if let Some(menu) = guard.as_ref() {
        let _ = menu.show.set_text(lang.show());
        let _ = menu
            .toggle_listening
            .set_text(listening_label(lang, listening));
        let _ = menu.quit.set_text(lang.quit());
    }
}

pub fn create_tray(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let state = app_handle.state::<AppState>();
    let config = state.config.lock().clone().unwrap_or_default();
    let listening = *state.listening.lock();
    let lang = lang_of(&config.lang);

    let show_item = MenuItemBuilder::new(lang.show())
        .id("show")
        .build(&app_handle)?;

    let toggle_listening_item = MenuItemBuilder::new(listening_label(lang, listening))
        .id("toggle_listening")
        .build(&app_handle)?;

    let quit_item = MenuItemBuilder::new(lang.quit())
        .id("quit")
        .build(&app_handle)?;

    let separator = PredefinedMenuItem::separator(&app_handle)?;

    let menu = MenuBuilder::new(&app_handle)
        .items(&[&show_item, &separator, &toggle_listening_item, &separator, &quit_item])
        .build()?;

    *state.tray_menu.lock() = Some(TrayMenu {
        show: show_item,
        toggle_listening: toggle_listening_item,
        quit: quit_item,
    });

    let icon = load_icon();

    let app_handle_clone1 = app_handle.clone();
    let app_handle_clone2 = app_handle.clone();
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |_app_handle, event| {
            let app_handle = app_handle_clone1.clone();
            match event.id.as_ref() {
                "show" => show_main_window(&app_handle),
                "toggle_listening" => {
                    let new_state = crate::commands::toggle_listening_state(&app_handle);
                    app_handle.emit("listening-changed", new_state).ok();
                }
                "quit" => {
                    // Release the click thread so a pending SendInput pair
                    // can't leave the synthetic mouse button stuck down.
                    crate::hotkeys::stop_engine(&app_handle);
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(move |_, event| {
            let app_handle = app_handle_clone2.clone();
            if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                show_main_window(&app_handle);
            }
        })
        .build(&app_handle)?;

    Ok(())
}

fn show_main_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
