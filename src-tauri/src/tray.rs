use tauri::{AppHandle, Manager, Emitter, image::Image};
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};

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
        .accelerator("F11")
        .build(&app_handle)?;

    let start_item = MenuItemBuilder::new("Start Clicking")
        .id("start")
        .accelerator("F9")
        .build(&app_handle)?;

    let stop_item = MenuItemBuilder::new("Stop Clicking")
        .id("stop")
        .accelerator("F10")
        .enabled(false)
        .build(&app_handle)?;

    let separator = PredefinedMenuItem::separator(&app_handle)?;

    let quit_item = MenuItemBuilder::new("Quit")
        .id("quit")
        .accelerator("Ctrl+Q")
        .build(&app_handle)?;

    let menu = MenuBuilder::new(&app_handle)
        .items(&[&show_item, &separator, &start_item, &stop_item, &separator, &quit_item])
        .build()?;

    let icon = create_default_icon();

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
                "start" => {
                    app_handle.emit("start-click-hotkey", ()).ok();
                }
                "stop" => {
                    app_handle.emit("stop-click-hotkey", ()).ok();
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