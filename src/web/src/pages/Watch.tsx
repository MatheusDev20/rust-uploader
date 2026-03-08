import { useNavigate } from 'react-router-dom'
import { useQueryParams } from '../hooks/params'
import type { WatchParams } from '../@types'
import { useVideo } from '../query/videos/by-id'

const MOCK_RELATED = [
  {
    id: '2',
    title: 'Help Center Theming from Scratch',
    channel: 'devdesk',
    views: 98100,
    duration: '24:10',
    tag: 'Help Center',
  },
  {
    id: '6',
    title: 'Apps Framework v2: Events and Locations',
    channel: 'devdesk',
    views: 87600,
    duration: '28:47',
    tag: 'Apps Framework',
  },
  {
    id: '5',
    title: 'Zendesk REST API: Pagination & Rate Limits',
    channel: 'devdesk',
    views: 201000,
    duration: '20:33',
    tag: 'REST API',
  },
  {
    id: '7',
    title: 'Trigger + Webhook Automation Deep Dive',
    channel: 'devdesk',
    views: 63400,
    duration: '16:55',
    tag: 'Automation',
  },
]

function formatViews(n: number) {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`
  if (n >= 1_000) return `${(n / 1_000).toFixed(0)}K`
  return n.toString()
}

export default function Watch() {
  const navigate = useNavigate()
  const { params } = useQueryParams<WatchParams>()

  const { d } = params
  const { metadata, playBackUrl, error, loading } = useVideo(d)

  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      {/* Top bar */}
      <header className="sticky top-0 z-10 bg-zinc-950/90 backdrop-blur border-b border-zinc-800 px-6 py-3 flex items-center gap-4">
        <button
          onClick={() => navigate('/')}
          className="font-mono font-bold text-[#00a650] hover:text-orange-400 transition-colors tracking-tight"
        >
          devdesk
        </button>
        <a
          href="#"
          className="ml-auto shrink-0 font-mono text-xs text-zinc-500 hover:text-white transition-colors"
        >
          about
        </a>
      </header>

      <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col lg:flex-row gap-8">
        {/* Player + metadata */}
        <div className="flex flex-col gap-4 flex-1 min-w-0">
          <div className="w-full aspect-video bg-zinc-900 rounded-lg overflow-hidden border border-zinc-800 flex items-center justify-center">
            {loading && <p className="font-mono text-zinc-600 text-sm">loading...</p>}
            {error && <p className="font-mono text-red-500 text-sm">error loading video.</p>}
            {playBackUrl && (
              <video controls className="w-full h-full object-contain">
                <source src={playBackUrl} type="video/mp4" />
              </video>
            )}
            {!loading && !error && !playBackUrl && (
              <span className="font-mono text-zinc-700 text-sm">no source</span>
            )}
          </div>

          {metadata && (
            <div className="flex flex-col gap-3">
              <h1 className="text-lg font-semibold leading-snug">{metadata.title}</h1>
              <div className="flex items-center gap-2 pb-3 border-b border-zinc-800">
                <span className="font-mono text-xs text-[#00a650]">#Sidebar</span>
                <span className="text-zinc-700">·</span>
                {/* <span className="font-mono text-xs text-zinc-500">{metadata.channel}</span> */}
                <span className="text-zinc-700">·</span>
                <span className="font-mono text-xs text-zinc-500">13,203 views</span>
              </div>
            </div>
          )}
        </div>

        {/* Related */}
        <aside className="w-full lg:w-72 shrink-0 flex flex-col gap-1">
          <p className="font-mono text-xs text-zinc-600 mb-2">up next</p>
          {MOCK_RELATED.map((v) => (
            <button
              key={v.id}
              onClick={() => navigate(`/watch?id=${v.id}`)}
              className="flex gap-3 text-left group py-2 border-b border-zinc-900"
            >
              <div className="relative w-32 shrink-0 aspect-video bg-zinc-900 rounded overflow-hidden border border-zinc-800 flex items-center justify-center">
                <span className="text-zinc-700 text-xs font-mono">thumb</span>
                <span className="absolute bottom-1 right-1 bg-black/80 text-white font-mono text-xs px-1 py-0.5 rounded">
                  {v.duration}
                </span>
              </div>
              <div className="flex flex-col gap-1 pt-0.5">
                <span className="font-mono text-xs text-[#00a650]">#{v.tag}</span>
                <p className="text-xs font-medium leading-snug group-hover:text-zinc-300 transition-colors line-clamp-2">
                  {v.title}
                </p>
                <p className="font-mono text-xs text-zinc-600">
                  {v.channel} · {formatViews(v.views)}
                </p>
              </div>
            </button>
          ))}
        </aside>
      </main>
    </div>
  )
}
