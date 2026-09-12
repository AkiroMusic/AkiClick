use enigo::{Enigo, MouseControllable};
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Interval sleeps are broken into slices of this length so stop/exit respond
/// quickly even when the configured interval is 60 seconds.
const SLICE_MS: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickMode {
    Left = 0,
    Right = 1,
    Double = 2,
}

impl From<u32> for ClickMode {
    fn from(value: u32) -> Self {
        match value {
            0 => ClickMode::Left,
            1 => ClickMode::Right,
            2 => ClickMode::Double,
            _ => ClickMode::Left,
        }
    }
}

#[derive(Clone)]
pub struct ClickEngine {
    /// Atomic so `stop` can flip it without holding the handle lock — the
    /// worker must always be able to observe the flag to exit.
    running: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl ClickEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(
        &self,
        app_handle: AppHandle,
        mode: ClickMode,
        interval_ms: u32,
        count: u32,
    ) -> Result<(), String> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err("Already running".to_string());
        }

        let running = Arc::clone(&self.running);
        let interval = Duration::from_millis(interval_ms.max(1) as u64);
        let handle = thread::spawn(move || {
            let mut enigo = Enigo::new();
            let mut clicks_done = 0u32;
            let infinite = count == 0;

            while running.load(Ordering::SeqCst) && (infinite || clicks_done < count) {
                match mode {
                    ClickMode::Left => {
                        enigo.mouse_click(enigo::MouseButton::Left);
                    }
                    ClickMode::Right => {
                        enigo.mouse_click(enigo::MouseButton::Right);
                    }
                    ClickMode::Double => {
                        enigo.mouse_click(enigo::MouseButton::Left);
                        thread::sleep(Duration::from_millis(10));
                        enigo.mouse_click(enigo::MouseButton::Left);
                    }
                }

                if !infinite {
                    clicks_done += 1;
                }

                sleep_interruptible(&running, interval);
            }

            running.store(false, Ordering::SeqCst);
            // Emit unconditionally so the UI recovers both after a counted run
            // finishes naturally and after an explicit stop.
            app_handle
                .emit("click-state-changed", serde_json::json!({ "isRunning": false }))
                .ok();
        });

        *self.handle.lock() = Some(handle);
        Ok(())
    }

    /// Flip the flag first, then join outside of any lock: the worker needs
    /// no lock held by us to exit, so this can never deadlock.
    pub fn stop(&self) -> Result<(), String> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Err("Not running".to_string());
        }
        let handle = self.handle.lock().take();
        if let Some(handle) = handle {
            let _ = handle.join();
        }
        Ok(())
    }
}

fn sleep_interruptible(running: &AtomicBool, total: Duration) {
    let slice = Duration::from_millis(SLICE_MS);
    let mut remaining = total;
    while remaining > Duration::ZERO && running.load(Ordering::SeqCst) {
        let step = remaining.min(slice);
        thread::sleep(step);
        remaining = remaining.saturating_sub(step);
    }
}

impl Default for ClickEngine {
    fn default() -> Self {
        Self::new()
    }
}
