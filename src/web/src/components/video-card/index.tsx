import { useNavigate } from 'react-router-dom'
import type { Content } from '../../@types'

export function VideoCard({ item }: { item: Extract<Content, { type: 'videos' }> }) {
  const navigate = useNavigate()
  return (
    <button
      onClick={() => navigate(`/watch?id=${item.id}`)}
      className="flex flex-col gap-2 text-left group"
    >
      <div className="relative w-full aspect-video bg-zinc-900 rounded-lg overflow-hidden border border-zinc-800 flex items-center justify-center">
        <span className="text-zinc-700 text-xs font-mono">thumbnail</span>
        <span className="absolute bottom-2 right-2 bg-black/80 text-white font-mono text-xs px-1.5 py-0.5 rounded">
          {item.views} views
        </span>
        <span className="absolute top-2 left-2 bg-[#00a650]/20 text-[#00a650] font-mono text-xs px-1.5 py-0.5 rounded border border-[#00a650]/30">
          {item.type}
        </span>
      </div>
      <div className="flex flex-col gap-3">
        <div className="flex gap-2">
          {item.tags &&
            item.tags.map((tag) => (
              <span className="font-mono text-xs text-[#00a650]">#{tag}</span>
            ))}
        </div>

        <p className="text-sm font-medium leading-snug group-hover:text-zinc-300 transition-colors">
          {item.title}
        </p>

        {/* <p className="font-mono text-xs text-zinc-500">{item.id}</p> */}
      </div>
    </button>
  )
}
