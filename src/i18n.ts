export type Lang = 'en' | 'zh'

const YEAR = new Date().getFullYear()

const en = {
  mode: 'MODE',
  interval: 'INTERVAL (MS)',
  count: 'COUNT (∞=0)',
  start: 'START',
  stop: 'STOP',
  exit: 'EXIT',
  enableListening: 'Enable Listening',
  disableListening: 'Disable Listening',
  idle: 'Idle',
  listening: 'Listening...',
  clicking: 'Clicking...',
  about: 'About',
  version: 'Version',
  author: 'Author',
  description: 'A lightweight Windows auto-clicker built with Tauri v2, Rust, and React.',
  close: 'Close',
  copyright: `© ${YEAR} Akiro. All rights reserved.`,
  hotkeyHint: 'Press hotkey to start/stop clicking when listening',
  save: 'Save',
  saved: 'Saved',
  saveFailed: 'Save failed — try again.',
  listeningWarning: 'Stop listening to modify settings',
  switchLang: '切换到中文',
  themeToggle: 'Toggle theme',
  pinToggle: 'Toggle always on top',
  modeTooltip: 'Click mode: {mode}',
  intervalTooltip: 'Click interval in milliseconds',
  countTooltip: 'Number of clicks (0 = infinite)',
  hotkeyConflict: 'Multiple hotkeys are assigned to the same key',
}

export type Dict = typeof en

const zh: Dict = {
  mode: '模式',
  interval: '间隔 (毫秒)',
  count: '次数 (∞=0)',
  start: '开始',
  stop: '停止',
  exit: '退出',
  enableListening: '启用监听',
  disableListening: '禁用监听',
  idle: '空闲',
  listening: '监听中...',
  clicking: '点击中...',
  about: '关于',
  version: '版本',
  author: '作者',
  description: '一个轻量级的 Windows 自动点击器，使用 Tauri v2、Rust 和 React 构建。',
  close: '关闭',
  copyright: `© ${YEAR} Akiro. 保留所有权利。`,
  hotkeyHint: '启用监听后，按热键开始/停止点击',
  save: '保存',
  saved: '已保存',
  saveFailed: '保存失败，请重试。',
  listeningWarning: '请先停止监听再修改设置',
  switchLang: 'Switch to English',
  themeToggle: '切换主题',
  pinToggle: '切换窗口置顶',
  modeTooltip: '点击模式：{mode}',
  intervalTooltip: '点击间隔（毫秒）',
  countTooltip: '点击次数（0 = 无限）',
  hotkeyConflict: '多个热键使用了相同的按键',
}

export const translations: Record<Lang, Dict> = { en, zh }

/** Coerce an untrusted stored language value into a valid Lang. */
export function toLang(value: string | null | undefined): Lang {
  return value === 'zh' ? 'zh' : 'en'
}
