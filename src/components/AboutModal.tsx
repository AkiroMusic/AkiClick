import { useEffect } from 'react'
import type { MouseEvent } from 'react'
import { open } from '@tauri-apps/plugin-shell'
import { XIcon } from '../icons'
import type { Dict } from '../i18n'

const WEBSITE_URL = 'https://akiromusic.com'

interface AboutModalProps {
  t: Dict
  version: string
  onClose: () => void
}

export function AboutModal({ t, version, onClose }: AboutModalProps) {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [onClose])

  // target="_blank" is blocked by the Tauri webview; the shell plugin opens
  // the link in the system browser instead.
  const handleOpenLink = (e: MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault()
    open(WEBSITE_URL).catch(console.error)
  }

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal-content glass-surface"
        role="dialog"
        aria-modal="true"
        aria-label={t.about}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="modal-header">
          <h2>{t.about} AkiClick</h2>
          <button className="icon-btn" onClick={onClose} aria-label={t.close} title={t.close}>
            <XIcon />
          </button>
        </div>
        <div className="modal-body">
          <img src="/icon.png" alt="AkiClick" className="about-logo" />
          <div className="about-version">
            {t.version} {version || '—'}
          </div>
          <p className="about-desc">{t.description}</p>
          <div className="about-author">
            <div className="author-name">{t.author}: Akiro</div>
            <a href={WEBSITE_URL} onClick={handleOpenLink} className="author-link">
              akiromusic.com
            </a>
          </div>
        </div>
      </div>
    </div>
  )
}
