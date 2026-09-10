use tauri::{AppHandle, Manager, Emitter, image::Image};
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};

/// Load icon embedded at compile time from icons/icon.png
fn load_icon() -> Image<'static> {
    let bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(bytes).expect("Failed to load embedded icon");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.into_raw();
    Image::new_owned(pixels, width, height)
}

pub fn create_tray(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItemBuilder::new("Show")
        .id("show")
        .build(&app_handle)?;

    let toggle_listening_item = MenuItemBuilder::new("Enable Listening")
        .id("toggle_listening")
        .build(&app_handle)?;

    let separator = PredefinedMenuItem::separator(&app_handle)?;

    let quit_item = MenuItemBuilder::new("Quit")
        .id("quit")
        .build(&app_handle)?;

    let menu = MenuBuilder::new(&app_handle)
        .items(&[&show_item, &separator, &toggle_listening_item, &separator, &quit_item])
        .build()?;

    let icon = load_icon();

    let app_handle_clone1 = app_handle.clone();
    let app_handle_clone2 = app_handle.clone();
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |_app_handle, event| {
            let app_handle = app_handle_clone1.clone();
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "toggle_listening" => {
                    let state = app_handle.state::<crate::AppState>();
                    let mut listening = state.listening.lock().unwrap();
                    *listening = !*listening;
                    let new_state = *listening;
                    drop(listening);
                    app_handle.emit("listening-changed", new_state).ok();
                }
                "quit" => {
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(move |_, event| {
            let app_handle = app_handle_clone2.clone();
            if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(&app_handle)?;

    Ok(())
}