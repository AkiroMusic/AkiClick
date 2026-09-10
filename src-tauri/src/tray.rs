use tauri::{AppHandle, Manager, Emitter, image::Image};
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use std::path::PathBuf;

/// Load icon from file or create default
fn load_icon() -> Image<'static> {
    // Try to load icon from various locations
    let possible_paths = vec![
        "icon.png".to_string(),
        "icons/icon.png".to_string(),
    ];
    
    // Also check exe directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            for name in &["icon.png", "icons/icon.png"] {
                let path = exe_dir.join(name);
                if path.exists() {
                    if let Ok(img) = load_image_from_file(&path) {
                        return img;
                    }
                }
            }
        }
    }
    
    // Check current directory
    for path_str in &possible_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            if let Ok(img) = load_image_from_file(&path) {
                return img;
            }
        }
    }
    
    // Fallback to default icon
    create_default_icon()
}

/// Load PNG image from file
fn load_image_from_file(path: &std::path::Path) -> Result<Image<'static>, Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;
    let img = image::load_from_memory(&data)?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let pixels = rgba.into_raw();
    Ok(Image::new_owned(pixels, width, height))
}

// Create a simple 32x32 RGBA icon (blue square with white cross)
fn create_default_icon() -> Image<'static> {
    let mut pixels = vec![0u8; 32 * 32 * 4];
    for y in 0..32 {
        for x in 0..32 {
            let idx = (y * 32 + x) * 4;
            // Blue background
            pixels[idx] = 13;       // R
            pixels[idx + 1] = 110;  // G
            pixels[idx + 2] = 253;  // B
            pixels[idx + 3] = 255;  // A
            // White cross
            if (x >= 14 && x <= 17) || (y >= 14 && y <= 17) {
                pixels[idx] = 255;
                pixels[idx + 1] = 255;
                pixels[idx + 2] = 255;
            }
        }
    }
    Image::new_owned(pixels, 32, 32)
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