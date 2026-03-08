import { useState } from 'react'
import { ContentCard } from '../components/ContentCard'
import { useFeed } from '../hooks/feed'

const CONTENT_TYPES = ['all', 'videos', 'article', 'snippet', 'guide', 'usefull APIs']

export default function Home() {
  const [activeType, setActiveType] = useState('All')
  const [query, setQuery] = useState('')

  const { content } = useFeed()

  const filtered = content.filter((item) => {
    const matchesType = activeType === 'All' || item.type === activeType
    const matchesQuery = item.title.toLowerCase().includes(query.toLowerCase())
    return matchesType && matchesQuery
  })

  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      {/* Top bar */}
      <header className="sticky top-0 z-10 bg-zinc-950/90 backdrop-blur border-b border-zinc-800 px-6 py-3 flex items-center gap-6">
        <span className="font-mono font-bold text-[#00a650] tracking-tight shrink-0">devdesk</span>
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="search anything..."
          className="w-full max-w-sm bg-zinc-900 border border-zinc-700 rounded px-3 py-1.5 text-sm font-mono outline-none focus:border-[#00a650]/60 placeholder:text-zinc-600 transition-colors"
        />
        <a
          href="#"
          className="ml-auto shrink-0 font-mono text-xs text-zinc-500 hover:text-white transition-colors"
        >
          about
        </a>
      </header>

      {/* Filters */}
      <div className="px-6 py-2 flex flex-col mt-6 gap-0">
        {/* Content type */}
        <div className="flex gap-4 py-2">
          {CONTENT_TYPES.map((t) => (
            <button
              key={t}
              onClick={() => setActiveType(t)}
              className={`font-mono text-xs transition-colors pb-1 border-b-2 ${
                activeType === t
                  ? 'text-white border-[#00a650]'
                  : 'text-zinc-500 border-transparent hover:text-zinc-300'
              }`}
            >
              {t}
            </button>
          ))}
        </div>
        {/* Topic tags */}
        {/* <div className="flex gap-2 py-2 overflow-x-auto scrollbar-none">
          {TAGS.map((tag) => (
            <button
              key={tag}
              onClick={() => setActiveTag(tag)}
              className={`shrink-0 px-3 py-1 rounded text-xs font-mono transition-colors ${
                activeTag === tag ? 'bg-[#00a650] text-white' : 'text-zinc-400 hover:text-white'
              }`}
            >
              {tag}
            </button>
          ))}
        </div> */}
      </div>

      <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col gap-6">
        {filtered.length === 0 && (
          <p className="font-mono text-sm text-zinc-600">no results found.</p>
        )}

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
          {filtered.map((item) => (
            <ContentCard key={item.id} item={item} />
          ))}
        </div>
      </main>
    </div>
  )
}
