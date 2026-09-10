import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Event } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

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

interface ClickState {
  isRunning: boolean
}

const CLICK_MODES = [
  { value: 0, label: 'Left' },
  { value: 1, label: 'Right' },
  { value: 2, label: 'Double' }
] as const

function App() {
  const [config, setConfig] = useState<Config | null>(null)
  const [clickState, setClickState] = useState<ClickState>({ isRunning: false })
  const [theme, setTheme] = useState<'light' | 'dark'>('dark')
  const [alwaysOnTop, setAlwaysOnTop] = useState(true)

  useEffect(() => {
    invoke<Config>('get_config').then((cfg) => setConfig(cfg)).catch(console.error)

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

    const unlistenClick = listen<ClickState>('click-state-changed', (event: Event<ClickState>) => setClickState(event.payload))
    const unlistenConfig = listen<Config>('config-changed', (event: Event<Config>) => setConfig(event.payload))

    return () => {
      unlistenClick.then((fn: () => void) => fn())
      unlistenConfig.then((fn: () => void) => fn())
    }
  }, [])

  const handleStartClick = async () => {
    if (!config) return
    await invoke('start_clicking', { mode: config.mode, intervalMs: config.freq, count: config.clicktimes })
    setClickState({ isRunning: true })
  }

  const handleStopClick = async () => {
    await invoke('stop_clicking')
    setClickState({ isRunning: false })
  }

  const handleToggleTheme = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
    localStorage.setItem('theme', newTheme)
    document.documentElement.dataset.theme = newTheme
  }

  const handleToggleAlwaysOnTop = async () => {
    const newValue = !alwaysOnTop
    setAlwaysOnTop(newValue)
    const window = getCurrentWebviewWindow()
    await window.setAlwaysOnTop(newValue)
  }

  const handleSaveConfig = async (newConfig: Partial<Config>) => {
    const updatedConfig = { ...config!, ...newConfig }
    await invoke('save_config', { config: updatedConfig })
    setConfig(updatedConfig)
  }

  if (!config) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="w-6 h-6 border-2 border-accent border-t-transparent rounded-full animate-spin" />
      </div>
    )
  }

  return (
    <div className="app-root">
      <div className="ambient-bg" />
      <div className="noise-overlay" />

      {/* Header */}
      <header className="app-header glass-surface">
        <span className="app-title">AkiClick</span>
        <div className="header-actions">
          <button
            onClick={handleToggleTheme}
            className="icon-btn"
            aria-label={`Switch to ${theme === 'light' ? 'dark' : 'light'} theme`}
          >
            {theme === 'light' ? '🌙' : '☀️'}
          </button>
          <button
            onClick={handleToggleAlwaysOnTop}
            className={`icon-btn ${alwaysOnTop ? 'active' : ''}`}
            aria-label={alwaysOnTop ? 'Disable always on top' : 'Enable always on top'}
          >
            📌
          </button>
        </div>
      </header>

      {/* Content */}
      <main className="app-content">
        {/* Mode */}
        <section>
          <div className="section-label">MODE</div>
          <div className="segmented-group">
            {CLICK_MODES.map((mode) => (
              <button
                key={mode.value}
                onClick={() => handleSaveConfig({ mode: mode.value })}
                className={`seg-btn ${config.mode === mode.value ? 'active' : ''}`}
              >
                {mode.label}
              </button>
            ))}
          </div>
        </section>

        {/* Interval & Count */}
        <section className="two-col">
          <div>
            <div className="section-label">INTERVAL (MS)</div>
            <input
              type="number"
              min="1"
              max="60000"
              value={config.freq}
              onChange={(e) => handleSaveConfig({ freq: parseInt(e.target.value) || 1 })}
              className="num-input"
            />
          </div>
          <div>
            <div className="section-label">COUNT (∞=0)</div>
            <input
              type="number"
              min="0"
              max="999999"
              value={config.clicktimes}
              onChange={(e) => handleSaveConfig({ clicktimes: parseInt(e.target.value) || 0 })}
              className="num-input"
            />
          </div>
        </section>

        {/* Hotkeys */}
        <section className="three-col">
          <div>
            <div className="section-label">START</div>
            <input
              type="text"
              value={config.left.toString()}
              onChange={(e) => handleSaveConfig({ left: parseInt(e.target.value) || 120 })}
              className="hk-input"
              maxLength={3}
            />
          </div>
          <div>
            <div className="section-label">STOP</div>
            <input
              type="text"
              value={config.right.toString()}
              onChange={(e) => handleSaveConfig({ right: parseInt(e.target.value) || 121 })}
              className="hk-input"
              maxLength={3}
            />
          </div>
          <div>
            <div className="section-label">EXIT</div>
            <input
              type="text"
              value={config.stop.toString()}
              onChange={(e) => handleSaveConfig({ stop: parseInt(e.target.value) || 122 })}
              className="hk-input"
              maxLength={3}
            />
          </div>
        </section>

        <div className="sep" />

        {/* Actions */}
        <section className="two-col">
          <button
            onClick={handleStartClick}
            disabled={clickState.isRunning}
            className="act-btn primary"
          >
            ▶ Start (F9)
          </button>
          <button
            onClick={handleStopClick}
            disabled={!clickState.isRunning}
            className="act-btn danger"
          >
            ⏹ Stop (F10)
          </button>
        </section>

        {/* Status */}
        <div className={`status ${clickState.isRunning ? 'running' : ''}`}>
          <span className={`dot ${clickState.isRunning ? 'on' : ''}`} />
          {clickState.isRunning ? 'Clicking…' : 'Idle'}
        </div>

        {/* Footer */}
        <footer className="footer">
          <span>v1.0.0</span>
          <span>F9·F10·F11</span>
        </footer>
      </main>
    </div>
  )
}

export default App