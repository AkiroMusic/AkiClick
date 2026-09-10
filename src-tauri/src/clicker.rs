use enigo::{Enigo, MouseControllable};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
    running: Arc<Mutex<bool>>,
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl ClickEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, mode: ClickMode, interval_ms: u32, count: u32) -> Result<(), String> {
        let mut running = self.running.lock();
        if *running {
            return Err("Already running".to_string());
        }
        *running = true;

        let running_clone = self.running.clone();
        let handle = thread::spawn(move || {
            let mut enigo = Enigo::new();
            let interval = Duration::from_millis(interval_ms as u64);
            let mut clicks_done = 0u32;
            let infinite = count == 0;

            while *running_clone.lock() && (infinite || clicks_done < count) {
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

                thread::sleep(interval);
            }

            *running_clone.lock() = false;
        });

        *self.handle.lock() = Some(handle);
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut running = self.running.lock();
        if !*running {
            return Err("Not running".to_string());
        }
        *running = false;

        if let Some(handle) = self.handle.lock().take() {
            let _ = handle.join();
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock()
    }
}

impl Default for ClickEngine {
    fn default() -> Self {
        Self::new()
    }
}