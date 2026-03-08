import { useState } from 'react'

export default function Home() {
  const [query, setQuery] = useState('')

  return (
    <div className="min-h-screen bg-zinc-950 text-white flex flex-col items-center px-4 py-8 gap-8">
      <div className="w-full max-w-2xl">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search videos..."
          className="w-full bg-zinc-800 border border-zinc-700 rounded-full px-5 py-3 text-sm outline-none focus:border-zinc-500 placeholder:text-zinc-500 transition-colors"
        />
      </div>

      <div className="w-full max-w-5xl">
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
          {/* {filtered?.map(video => (
            <button
              key={video.id}
              onClick={() => navigate(`/watch?id=${video.id}`)}
              className="flex flex-col gap-2 text-left bg-zinc-900 rounded-xl overflow-hidden border border-zinc-800 hover:border-zinc-600 transition-colors"
            >
              <div className="w-full aspect-video bg-zinc-800 flex items-center justify-center">
                <span className="text-zinc-600 text-xs">No thumbnail</span>
              </div>
              <div className="px-3 pb-3 flex flex-col gap-1">
                <p className="text-sm font-medium leading-snug">{video.title}</p>
                <p className="text-xs text-zinc-400">{video.views.toLocaleString()} views</p>
              </div>
            </button>
          ))} */}
        </div>
      </div>
    </div>
  )
}
