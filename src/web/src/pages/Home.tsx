import { useState, useEffect, useRef } from 'react'
import { useResourceTypes } from '../hooks/doc-types'

export default function Home() {
  const [activeType, setActiveType] = useState('All')
  const [query, setQuery] = useState('')
  const [searchOpen, setSearchOpen] = useState(false)
  const modalInputRef = useRef<HTMLInputElement>(null)

  const { resourceTypes, isLoading: loadingResourceTypes } = useResourceTypes()

  function openSearch() {
    setSearchOpen(true)
  }

  function closeSearch() {
    setSearchOpen(false)
  }

  // Focus modal input when it opens
  useEffect(() => {
    if (searchOpen) {
      setTimeout(() => modalInputRef.current?.focus(), 30)
    }
  }, [searchOpen])

  // Close on Escape
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') closeSearch()
      // Open with Ctrl+K / Cmd+K like docs sites
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault()
        openSearch()
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      {/* Top bar */}
      <header className="sticky top-0 z-10 bg-zinc-950/90 backdrop-blur border-b border-zinc-800 px-6 py-3 flex items-center gap-6">
        <span className="font-mono font-bold text-[#00a650] tracking-tight shrink-0">devdesk</span>

        <a
          href="#"
          className="ml-auto shrink-0 font-mono text-xs text-zinc-500 hover:text-white transition-colors"
        >
          about
        </a>
      </header>

      {/* Centered search trigger + tag navigation */}
      <div className="flex flex-col items-center px-6 mt-16 mb-8 gap-4">
        <button
          onClick={openSearch}
          className="w-full max-w-2xl bg-zinc-900 border border-zinc-700 rounded-xl px-5 py-4 text-base font-mono text-zinc-500 text-left flex items-center gap-3 hover:border-zinc-500 transition-colors shadow-lg"
        >
          <svg
            className="w-5 h-5 shrink-0"
            fill="none"
            stroke="currentColor"
            strokeWidth={2}
            viewBox="0 0 24 24"
          >
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.35-4.35" />
          </svg>
          <span className="flex-1">search for anything...</span>
          <kbd className="hidden sm:inline-flex items-center gap-0.5 px-2 py-1 rounded bg-zinc-800 border border-zinc-700 text-xs text-zinc-400 font-mono">
            ⌘K
          </kbd>
        </button>
        <span className="font-mono text-md mt-4">or starting exploring by</span>
        <div className="px-6 py-2 flex flex-col gap-0">
          {loadingResourceTypes ? <p>carregando ...</p> : null}

          <div className="flex gap-4 py-2">
            {resourceTypes
              ? resourceTypes.map(({ id, display, name }) => (
                  <button
                    key={id}
                    onClick={() => setActiveType(name)}
                    className={`font-mono text-xs cursor-pointer transition-colors pb-1 border-b-2 ${
                      activeType === name
                        ? 'text-white border-[#00a650]'
                        : 'text-zinc-500 border-transparent hover:text-zinc-300'
                    }`}
                  >
                    {display}
                  </button>
                ))
              : []}
          </div>
        </div>
      </div>

      {/* <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col gap-6">
        {filtered.length === 0 && (
          <p className="font-mono text-sm text-zinc-600">no results found.</p>
        )}

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
          {filtered.map((item) => (
            <ContentCard key={item.id} item={item} />
          ))}
        </div>
      </main> */}

      {/* Search modal */}
      {searchOpen && (
        <div
          className="fixed inset-0 z-50 flex items-start justify-center pt-[15vh] px-4"
          onMouseDown={(e) => {
            // Close when clicking the backdrop (not the modal itself)
            if (e.target === e.currentTarget) closeSearch()
          }}
        >
          {/* Backdrop */}
          <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={closeSearch} />

          {/* Modal panel */}
          <div className="relative w-full max-w-xl bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl overflow-hidden">
            {/* Input row */}
            <div className="flex items-center gap-3 px-4 py-3 border-b border-zinc-800">
              <svg
                className="w-4 h-4 text-zinc-400 shrink-0"
                fill="none"
                stroke="currentColor"
                strokeWidth={2}
                viewBox="0 0 24 24"
              >
                <circle cx="11" cy="11" r="8" />
                <path d="m21 21-4.35-4.35" />
              </svg>
              <input
                ref={modalInputRef}
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="search anything..."
                className="flex-1 bg-transparent text-sm font-mono text-white placeholder:text-zinc-600 outline-none"
              />
              {query && (
                <button
                  onClick={() => setQuery('')}
                  className="text-zinc-500 hover:text-white transition-colors text-xs font-mono"
                >
                  clear
                </button>
              )}
              <kbd
                onClick={closeSearch}
                className="cursor-pointer px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 text-[10px] text-zinc-400 font-mono"
              >
                esc
              </kbd>
            </div>

            {/* Results */}
            <div className="max-h-[50vh] overflow-y-auto">
              {query.trim() === '' && (
                <p className="px-4 py-6 text-center font-mono text-xs text-zinc-600">
                  start typing to search...
                </p>
              )}

              {query.trim() !== '' && (
                <p className="px-4 py-6 text-center font-mono text-xs text-zinc-600">
                  no results for "{query}"
                </p>
              )}
            </div>

            {/* Footer hint */}
            <div className="px-4 py-2 border-t border-zinc-800 flex gap-4">
              <span className="font-mono text-[10px] text-zinc-600 flex items-center gap-1">
                <kbd className="px-1 py-0.5 rounded bg-zinc-800 border border-zinc-700">↑↓</kbd>{' '}
                navigate
              </span>
              <span className="font-mono text-[10px] text-zinc-600 flex items-center gap-1">
                <kbd className="px-1 py-0.5 rounded bg-zinc-800 border border-zinc-700">↵</kbd> open
              </span>
              <span className="font-mono text-[10px] text-zinc-600 flex items-center gap-1">
                <kbd className="px-1 py-0.5 rounded bg-zinc-800 border border-zinc-700">esc</kbd>{' '}
                close
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
