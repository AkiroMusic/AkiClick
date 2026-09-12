/**
 * Mirrors `Config` in src-tauri/src/config.rs — keep field names in sync.
 * Legacy names (left/right/stop/freq/clicktimes) follow the 鼠标连点器 ("REF")
 * INI format: left = start hotkey, right = stop hotkey, stop = exit hotkey.
 */
export interface Config {
  mode: number
  freq: number
  clicktimes: number
  clickstate: number
  showstate: number
  left: number
  right: number
  stop: number
  theme: string
  lang: string
  always_on_top: number
}
