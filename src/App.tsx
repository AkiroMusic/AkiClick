import { useEffect, useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Event } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

// ==================== Types ====================
interface Config {
  mode: number
  freq: number
  clicktimes: number
  clickstate: number
  showstate: number
  left: number
  right: number
  stop: number
}

// ==================== i18n ====================
const translations = {
  en: {
    mode: 'MODE',
    interval: 'INTERVAL (MS)',
    count: 'COUNT (∞=0)',
    hotkeys: 'HOTKEYS',
    start: 'START',
    stop: 'STOP',
    exit: 'EXIT',
    enableListening: 'Enable Listening',
    disableListening: 'Disable Listening',
    idle: 'Idle',
    listening: 'Listening...',
    clicking: 'Clicking...',
    left: 'Left',
    right: 'Right',
    double: 'Double',
    about: 'About',
    version: 'Version',
    author: 'Author',
    description: 'A lightweight Windows auto-clicker built with Tauri v2, Rust, and React.',
    close: 'Close',
    copyright: '© 2024 Akiro. All rights reserved.',
    hotkeyHint: 'Press hotkey to start/stop clicking when listening',
  },
  zh: {
    mode: '模式',
    interval: '间隔 (毫秒)',
    count: '次数 (∞=0)',
    hotkeys: '热键',
    start: '开始',
    stop: '停止',
    exit: '退出',
    enableListening: '启用监听',
    disableListening: '禁用监听',
    idle: '空闲',
    listening: '监听中...',
    clicking: '点击中...',
    left: '左键',
    right: '右键',
    double: '双击',
    about: '关于',
    version: '版本',
    author: '作者',
    description: '一个轻量级的 Windows 自动点击器，使用 Tauri v2、Rust 和 React 构建。',
    close: '关闭',
    copyright: '© 2024 Akiro. 保留所有权利。',
    hotkeyHint: '启用监听后，按热键开始/停止点击',
  }
}

// ==================== SVG Icons ====================
const SunIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="5"/>
    <line x1="12" y1="1" x2="12" y2="3"/>
    <line x1="12" y1="21" x2="12" y2="23"/>
    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
    <line x1="1" y1="12" x2="3" y2="12"/>
    <line x1="21" y1="12" x2="23" y2="12"/>
    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
  </svg>
)

const MoonIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
  </svg>
)

const PinIcon = ({ active }: { active: boolean }) => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill={active ? 'currentColor' : 'none'} stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M12 2L12 22"/>
    <path d="M5 12L12 5L19 12"/>
    <circle cx="12" cy="5" r="2"/>
  </svg>
)

const InfoIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="10"/>
    <line x1="12" y1="16" x2="12" y2="12"/>
    <line x1="12" y1="8" x2="12.01" y2="8"/>
  </svg>
)

const XIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <line x1="18" y1="6" x2="6" y2="18"/>
    <line x1="6" y1="6" x2="18" y2="18"/>
  </svg>
)

const LangIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="10"/>
    <line x1="2" y1="12" x2="22" y2="12"/>
    <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
  </svg>
)

const CLICK_MODES = [
  { value: 0, labelEn: 'Left', labelZh: '左键' },
  { value: 1, labelEn: 'Right', labelZh: '右键' },
  { value: 2, labelEn: 'Double', labelZh: '双击' }
] as const

function App() {
  const [config, setConfig] = useState<Config | null>(null)
  const [listening, setListening] = useState(false)
  const [isClicking, setIsClicking] = useState(false)
  const [theme, setTheme] = useState<'light' | 'dark'>('dark')
  const [alwaysOnTop, setAlwaysOnTop] = useState(true)
  const [lang, setLang] = useState<'en' | 'zh'>('en')
  const [showAbout, setShowAbout] = useState(false)
  const [version, setVersion] = useState('1.0.0')

  const t = translations[lang]

  // Load config and settings
  useEffect(() => {
    invoke<Config>('get_config').then((cfg) => setConfig(cfg)).catch(console.error)
    invoke<string>('get_version').then((v) => setVersion(v)).catch(console.error)

    // Load theme
    const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | null
    if (savedTheme) {
      setTheme(savedTheme)
      document.documentElement.dataset.theme = savedTheme
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      setTheme('dark')
      document.documentElement.dataset.theme = 'dark'
    } else {
      setTheme('light')
      document.documentElement.dataset.theme = 'light'
    }

    // Load language
    const savedLang = localStorage.getItem('lang') as 'en' | 'zh' | null
    if (savedLang) {
      setLang(savedLang)
    }

    // Load always on top
    const savedAlwaysOnTop = localStorage.getItem('alwaysOnTop')
    if (savedAlwaysOnTop !== null) {
      setAlwaysOnTop(savedAlwaysOnTop === 'true')
    }
  }, [])

  // Event listeners
  useEffect(() => {
    const unlistenListening = listen<boolean>('listening-changed', (event: Event<boolean>) => {
      setListening(event.payload)
    })

    const unlistenClick = listen<{ isRunning: boolean }>('click-state-changed', (event: Event<{ isRunning: boolean }>) => {
      setIsClicking(event.payload.isRunning)
    })

    const unlistenConfig = listen<Config>('config-changed', (event: Event<Config>) => {
      setConfig(event.payload)
    })

    return () => {
      unlistenListening.then((fn: () => void) => fn())
      unlistenClick.then((fn: () => void) => fn())
      unlistenConfig.then((fn: () => void) => fn())
    }
  }, [])

  // Toggle listening state
  const handleToggleListening = useCallback(async () => {
    const newState = await invoke<boolean>('toggle_listening')
    setListening(newState)
  }, [])

  // Toggle theme
  const handleToggleTheme = useCallback(() => {
    const newTheme = theme === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
    localStorage.setItem('theme', newTheme)
    document.documentElement.dataset.theme = newTheme
  }, [theme])

  // Toggle always on top
  const handleToggleAlwaysOnTop = useCallback(async () => {
    const newValue = !alwaysOnTop
    setAlwaysOnTop(newValue)
    localStorage.setItem('alwaysOnTop', String(newValue))
    const window = getCurrentWebviewWindow()
    await window.setAlwaysOnTop(newValue)
  }, [alwaysOnTop])

  // Toggle language
  const handleToggleLang = useCallback(() => {
    const newLang = lang === 'en' ? 'zh' : 'en'
    setLang(newLang)
    localStorage.setItem('lang', newLang)
  }, [lang])

  // Save config
  const handleSaveConfig = useCallback(async (newConfig: Partial<Config>) => {
    if (!config) return
    const updatedConfig = { ...config, ...newConfig }
    await invoke('save_config', { config: updatedConfig })
    setConfig(updatedConfig)
  }, [config])

  if (!config) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="w-6 h-6 border-2 border-[var(--acc)] border-t-transparent rounded-full animate-spin" />
      </div>
    )
  }

  const getStatusText = () => {
    if (isClicking) return t.clicking
    if (listening) return t.listening
    return t.idle
  }

  const getStatusClass = () => {
    if (isClicking) return 'status clicking'
    if (listening) return 'status listening'
    return 'status'
  }

  return (
    <div className="app-root">
      <div className="ambient-bg" />
      <div className="noise-overlay" />

      {/* Header */}
      <header className="app-header glass-surface">
        <span className="app-title">AkiClick</span>
        <div className="header-actions">
          {/* Language toggle */}
          <button
            onClick={handleToggleLang}
            className="icon-btn"
            title={lang === 'en' ? '切换到中文' : 'Switch to English'}
          >
            <LangIcon />
          </button>
          {/* Theme toggle */}
          <button
            onClick={handleToggleTheme}
            className="icon-btn"
            title={lang === 'en' ? 'Toggle theme' : '切换主题'}
          >
            {theme === 'light' ? <MoonIcon /> : <SunIcon />}
          </button>
          {/* Always on top */}
          <button
            onClick={handleToggleAlwaysOnTop}
            className={`icon-btn ${alwaysOnTop ? 'active' : ''}`}
            title={lang === 'en' ? 'Toggle always on top' : '切换窗口置顶'}
          >
            <PinIcon active={alwaysOnTop} />
          </button>
          {/* Info */}
          <button
            onClick={() => setShowAbout(true)}
            className="icon-btn"
            title={t.about}
          >
            <InfoIcon />
          </button>
        </div>
      </header>

      {/* Content */}
      <main className="app-content">
        {/* Mode */}
        <section>
          <div className="section-label">{t.mode}</div>
          <div className="segmented-group">
            {CLICK_MODES.map((mode) => (
              <button
                key={mode.value}
                onClick={() => handleSaveConfig({ mode: mode.value })}
                className={`seg-btn ${config.mode === mode.value ? 'active' : ''}`}
                title={lang === 'en' ? `Click mode: ${mode.labelEn}` : `点击模式: ${mode.labelZh}`}
              >
                {lang === 'en' ? mode.labelEn : mode.labelZh}
              </button>
            ))}
          </div>
        </section>

        {/* Interval & Count */}
        <section className="two-col">
          <div>
            <div className="section-label">{t.interval}</div>
            <input
              type="number"
              min="1"
              max="60000"
              value={config.freq}
              onChange={(e) => handleSaveConfig({ freq: parseInt(e.target.value) || 1 })}
              className="num-input"
              title={lang === 'en' ? 'Click interval in milliseconds' : '点击间隔（毫秒）'}
            />
          </div>
          <div>
            <div className="section-label">{t.count}</div>
            <input
              type="number"
              min="0"
              max="999999"
              value={config.clicktimes}
              onChange={(e) => handleSaveConfig({ clicktimes: parseInt(e.target.value) || 0 })}
              className="num-input"
              title={lang === 'en' ? 'Number of clicks (0 = infinite)' : '点击次数（0 = 无限）'}
            />
          </div>
        </section>

        {/* Hotkeys */}
        <section className="three-col">
          <div>
            <div className="section-label">{t.start}</div>
            <input
              type="text"
              value={config.left.toString()}
              onChange={(e) => handleSaveConfig({ left: parseInt(e.target.value) || 120 })}
              className="hk-input"
              maxLength={3}
              title={lang === 'en' ? 'Start hotkey VK code (120=F9)' : '开始热键 VK 码 (120=F9)'}
            />
          </div>
          <div>
            <div className="section-label">{t.stop}</div>
            <input
              type="text"
              value={config.right.toString()}
              onChange={(e) => handleSaveConfig({ right: parseInt(e.target.value) || 121 })}
              className="hk-input"
              maxLength={3}
              title={lang === 'en' ? 'Stop hotkey VK code (121=F10)' : '停止热键 VK 码 (121=F10)'}
            />
          </div>
          <div>
            <div className="section-label">{t.exit}</div>
            <input
              type="text"
              value={config.stop.toString()}
              onChange={(e) => handleSaveConfig({ stop: parseInt(e.target.value) || 122 })}
              className="hk-input"
              maxLength={3}
              title={lang === 'en' ? 'Exit hotkey VK code (122=F11)' : '退出热键 VK 码 (122=F11)'}
            />
          </div>
        </section>

        {/* Hotkey hint */}
        <div className="hotkey-hint">{t.hotkeyHint}</div>

        <div className="sep" />

        {/* Listening toggle button */}
        <section>
          <button
            onClick={handleToggleListening}
            className={`listen-btn ${listening ? 'active' : ''}`}
          >
            {listening ? t.disableListening : t.enableListening}
          </button>
        </section>

        {/* Status */}
        <div className={getStatusClass()}>
          <span className={`dot ${listening ? 'on' : ''} ${isClicking ? 'clicking' : ''}`} />
          {getStatusText()}
        </div>
      </main>

      {/* Footer */}
      <footer className="app-footer">
        <span>{t.copyright}</span>
      </footer>

      {/* About Modal */}
      {showAbout && (
        <div className="modal-overlay" onClick={() => setShowAbout(false)}>
          <div className="modal-content glass-surface" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>{t.about} AkiClick</h2>
              <button className="icon-btn" onClick={() => setShowAbout(false)}>
                <XIcon />
              </button>
            </div>
            <div className="modal-body">
              <div className="about-logo">AkiClick</div>
              <div className="about-version">{t.version} {version}</div>
              <p className="about-desc">{t.description}</p>
              <div className="about-author">
                <div className="author-name">{t.author}: Akiro</div>
                <a href="https://akiromusic.com" target="_blank" rel="noopener noreferrer" className="author-link">
                  akiromusic.com
                </a>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default App