//! Minimal backend-side i18n for the tray menu and error messages.
//! Keep the wording in sync with `src/i18n.ts` on the frontend.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Zh,
}

pub fn lang_of(code: &str) -> Lang {
    if code == "zh" {
        Lang::Zh
    } else {
        Lang::En
    }
}

impl Lang {
    pub fn show(self) -> &'static str {
        match self {
            Lang::En => "Show",
            Lang::Zh => "显示",
        }
    }

    pub fn quit(self) -> &'static str {
        match self {
            Lang::En => "Quit",
            Lang::Zh => "退出",
        }
    }

    pub fn enable_listening(self) -> &'static str {
        match self {
            Lang::En => "Enable Listening",
            Lang::Zh => "启用监听",
        }
    }

    pub fn disable_listening(self) -> &'static str {
        match self {
            Lang::En => "Disable Listening",
            Lang::Zh => "禁用监听",
        }
    }

    fn action_name(self, action: &str) -> &'static str {
        match (self, action) {
            (Lang::Zh, "start") => "开始",
            (Lang::Zh, "stop") => "停止",
            (Lang::Zh, _) => "退出",
            (Lang::En, "start") => "Start",
            (Lang::En, "stop") => "Stop",
            (Lang::En, _) => "Exit",
        }
    }

    pub fn hotkey_error(self, action: &str, key: &str) -> String {
        match self {
            Lang::En => format!(
                "Failed to register {} hotkey {}. It may already be in use by another application.",
                self.action_name(action),
                key
            ),
            Lang::Zh => format!(
                "「{}」热键 {} 注册失败，可能已被其他程序占用。",
                self.action_name(action),
                key
            ),
        }
    }

    pub fn hotkey_invalid(self, action: &str, vk: u32) -> String {
        match self {
            Lang::En => format!(
                "Invalid {} hotkey value (VK {}). The default hotkey was restored.",
                self.action_name(action),
                vk
            ),
            Lang::Zh => format!(
                "「{}」热键值无效（VK {}），已恢复为默认热键。",
                self.action_name(action),
                vk
            ),
        }
    }
}
