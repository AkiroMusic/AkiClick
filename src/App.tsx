import { useEffect, useState, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Event } from '@tauri-apps/api/event'
import { translations } from './i18n'
import type { Lang } from './i18n'
import { vkToName, nameToVk, HOTKEY_OPTIONS } from './vkCodes'
import { SunIcon, MoonIcon, PinIcon, InfoIcon, LangIcon, SaveIcon } from './icons'
import { AboutModal } from './components/AboutModal'
import type { Config } from './types'

// ==================== Helpers ====================

const MIN_INTERVAL = 1
const MAX_INTERVAL = 60000
const MAX_COUNT = 999999

/** Parse an input value and clamp it into [min, max]; NaN falls back. */
function parseClamped(raw: string, min: number, max: number, fallback: number): number {
  const n = parseInt(raw, 10)
  if (Number.isNaN(n)) return fallback
  return Math.min(max, Math.max(min, n))
}

const CLICK_MODES = [
  { value: 0, labelEn: 'Left', labelZh: '左键' },
  { value: 1, labelEn: 'Right', labelZh: '右键' },
  { value: 2, labelEn: 'Double', labelZh: '双击' }
] as const

function App() {
  const [config, setConfig] = useState<Config | null>(null)
  const [draft, setDraft] = useState<Config | null>(null)
  const [dirty, setDirty] = useState(false)
  const [showSaved, setShowSaved] = useState(false)
  const [saveFailed, setSaveFailed] = useState(false)
  const [listening, setListening] = useState(false)
  const [isClicking, setIsClicking] = useState(false)
  const [theme, setTheme] = useState<'light' | 'dark'>('dark')
  const [alwaysOnTop, setAlwaysOnTop] = useState(true)
  const [lang, setLang] = useState<Lang>('en')
  const [showAbout, setShowAbout] = useState(false)
  const [version, setVersion] = useState('')
  const [hotkeyWarning, setHotkeyWarning] = useState('')
  const savedTimer = useRef<number | null>(null)

  const t = translations[lang]
  const locked = listening

  // Load config and preferences (all persisted in shubiao.ini on the backend)
  useEffect(() => {
    invoke<Config>('get_config').then((cfg) => {
      setConfig(cfg)
      setDraft(cfg)
      applyTheme(cfg.theme === 'light' ? 'light' : 'dark')
      const l: Lang = cfg.lang === 'zh' ? 'zh' : 'en'
      setLang(l)
      document.documentElement.lang = l
      setAlwaysOnTop(cfg.always_on_top === 1)
    }).catch(console.error)
    invoke<string>('get_version').then((v) => setVersion(v)).catch(console.error)

    return () => {
      if (savedTimer.current !== null) window.clearTimeout(savedTimer.current)
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
      setDraft(event.payload)
      setDirty(false)
      setHotkeyWarning('')
    })
    const unlistenHotkeyError = listen<{ message: string }>('hotkey-error', (event: Event<{ message: string }>) => {
      setHotkeyWarning(event.payload.message)
    })
    return () => {
      unlistenListening.then((fn: () => void) => fn())
      unlistenClick.then((fn: () => void) => fn())
      unlistenConfig.then((fn: () => void) => fn())
      unlistenHotkeyError.then((fn: () => void) => fn())
    }
  }, [])

  const applyTheme = useCallback((next: 'light' | 'dark') => {
    setTheme(next)
    document.documentElement.dataset.theme = next
  }, [])

  // Update draft (local only, not saved until the Save button is used)
  const updateDraft = useCallback((patch: Partial<Config>) => {
    if (!draft || locked) return
    setDraft({ ...draft, ...patch })
    setDirty(true)
  }, [draft, locked])

  // Persist UI preferences immediately (separate from the settings draft)
  const updatePrefs = useCallback(async (patch: Partial<Config>) => {
    if (!config || !draft) return
    const nextConfig = { ...config, ...patch }
    const nextDraft = { ...draft, ...patch }
    setConfig(nextConfig)
    setDraft(nextDraft)
    try {
      await invoke('save_preferences', {
        theme: nextConfig.theme,
        lang: nextConfig.lang,
        alwaysOnTop: nextConfig.always_on_top,
      })
    } catch (e) {
      console.error(e)
    }
  }, [config, draft])

  // Save draft to backend; the backend returns the sanitized config
  const handleSave = useCallback(async () => {
    if (!draft) return
    try {
      const saved = await invoke<Config>('save_config', { config: draft })
      setConfig(saved)
      setDraft(saved)
      setDirty(false)
      setSaveFailed(false)
      setShowSaved(true)
      if (savedTimer.current !== null) window.clearTimeout(savedTimer.current)
      savedTimer.current = window.setTimeout(() => setShowSaved(false), 2000)
    } catch (e) {
      console.error(e)
      setSaveFailed(true)
    }
  }, [draft])

  // Toggle listening
  const handleToggleListening = useCallback(async () => {
    const newState = await invoke<boolean>('toggle_listening')
    setListening(newState)
  }, [])

  // Toggle theme
  const handleToggleTheme = useCallback(() => {
    const next = theme === 'light' ? 'dark' : 'light'
    applyTheme(next)
    void updatePrefs({ theme: next })
  }, [theme, applyTheme, updatePrefs])

  // Toggle always on top (the backend applies it to the window)
  const handleToggleAlwaysOnTop = useCallback(() => {
    const next = !alwaysOnTop
    setAlwaysOnTop(next)
    void updatePrefs({ always_on_top: next ? 1 : 0 })
  }, [alwaysOnTop, updatePrefs])

  // Toggle language
  const handleToggleLang = useCallback(() => {
    const next: Lang = lang === 'en' ? 'zh' : 'en'
    setLang(next)
    document.documentElement.lang = next
    void updatePrefs({ lang: next })
  }, [lang, updatePrefs])

  if (!config || !draft) {
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

  const hotkeyConflict =
    draft.left === draft.right || draft.left === draft.stop || draft.right === draft.stop

  return (
    <div className="app-root">
      <div className="ambient-bg" />
      <div className="noise-overlay" />

      {/* Header */}
      <header className="app-header glass-surface">
        <span className="app-title">AkiClick</span>
        <div className="header-actions">
          <button onClick={handleToggleLang} className="icon-btn"
            title={t.switchLang} aria-label={t.switchLang}>
            <LangIcon />
          </button>
          <button onClick={handleToggleTheme} className="icon-btn"
            title={t.themeToggle} aria-label={t.themeToggle}>
            {theme === 'light' ? <MoonIcon /> : <SunIcon />}
          </button>
          <button onClick={handleToggleAlwaysOnTop}
            className={`icon-btn ${alwaysOnTop ? 'active' : ''}`}
            title={t.pinToggle} aria-label={t.pinToggle}
            aria-pressed={alwaysOnTop}>
            <PinIcon active={alwaysOnTop} />
          </button>
          <button onClick={() => setShowAbout(true)} className="icon-btn"
            title={t.about} aria-label={t.about}>
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
                onClick={() => updateDraft({ mode: mode.value })}
                className={`seg-btn ${draft.mode === mode.value ? 'active' : ''} ${locked ? 'disabled' : ''}`}
                disabled={locked}
                title={t.modeTooltip.replace('{mode}', lang === 'en' ? mode.labelEn : mode.labelZh)}
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
              min={MIN_INTERVAL}
              max={MAX_INTERVAL}
              value={draft.freq}
              onChange={(e) => updateDraft({ freq: parseClamped(e.target.value, MIN_INTERVAL, MAX_INTERVAL, MIN_INTERVAL) })}
              className="num-input"
              disabled={locked}
              title={t.intervalTooltip}
              aria-label={t.intervalTooltip}
            />
          </div>
          <div>
            <div className="section-label">{t.count}</div>
            <input
              type="number"
              min={0}
              max={MAX_COUNT}
              value={draft.clicktimes}
              onChange={(e) => updateDraft({ clicktimes: parseClamped(e.target.value, 0, MAX_COUNT, 0) })}
              className="num-input"
              disabled={locked}
              title={t.countTooltip}
              aria-label={t.countTooltip}
            />
          </div>
        </section>

        {/* Hotkeys */}
        <section className="three-col">
          <div>
            <div className="section-label">{t.start}</div>
            <select
              value={vkToName(draft.left)}
              onChange={(e) => updateDraft({ left: nameToVk(e.target.value) })}
              className="hk-input"
              disabled={locked}
              title={t.start}
              aria-label={t.start}
            >
              {HOTKEY_OPTIONS.map((opt) => (
                <option key={opt.value} value={opt.label}>{opt.label}</option>
              ))}
            </select>
          </div>
          <div>
            <div className="section-label">{t.stop}</div>
            <select
              value={vkToName(draft.right)}
              onChange={(e) => updateDraft({ right: nameToVk(e.target.value) })}
              className="hk-input"
              disabled={locked}
              title={t.stop}
              aria-label={t.stop}
            >
              {HOTKEY_OPTIONS.map((opt) => (
                <option key={opt.value} value={opt.label}>{opt.label}</option>
              ))}
            </select>
          </div>
          <div>
            <div className="section-label">{t.exit}</div>
            <select
              value={vkToName(draft.stop)}
              onChange={(e) => updateDraft({ stop: nameToVk(e.target.value) })}
              className="hk-input"
              disabled={locked}
              title={t.exit}
              aria-label={t.exit}
            >
              {HOTKEY_OPTIONS.map((opt) => (
                <option key={opt.value} value={opt.label}>{opt.label}</option>
              ))}
            </select>
          </div>
        </section>

        {hotkeyConflict && (
          <div className="hotkey-hint error" role="alert">{t.hotkeyConflict}</div>
        )}
        {hotkeyWarning && (
          <div className="hotkey-hint error" role="alert">{hotkeyWarning}</div>
        )}

        {/* Hotkey hint */}
        <div className="hotkey-hint">{t.hotkeyHint}</div>

        <div className="sep" />

        {/* Save button / saved / error indicators */}
        <section>
          {locked ? (
            <div className="hotkey-hint">{t.listeningWarning}</div>
          ) : (
            <>
              {showSaved && (
                <div className="saved-indicator">
                  <SaveIcon /> {t.saved}
                </div>
              )}
              {dirty && (
                <button className="save-btn" onClick={handleSave}>{t.save}</button>
              )}
              {saveFailed && (
                <div className="hotkey-hint error" role="alert">{t.saveFailed}</div>
              )}
            </>
          )}
        </section>

        {/* Listening toggle button */}
        <section>
          <button
            onClick={handleToggleListening}
            className={`listen-btn ${listening ? 'active' : ''}`}
            aria-pressed={listening}
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
        <AboutModal t={t} version={version} onClose={() => setShowAbout(false)} />
      )}
    </div>
  )
}

export default App
