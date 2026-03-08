import { useNavigate } from 'react-router-dom'
import { useQueryParams } from '../hooks/params'
import type { WatchParams } from '../@types'
import { useVideo } from '../query/videos'

export default function Watch() {
  const navigate = useNavigate()
  const { params } = useQueryParams<WatchParams>()

  const { d } = params
  const { metadata, playBackUrl, error, loading } = useVideo(d)

  // if (!id) {
  //   navigate('/')
  //   return null
  // }

  return (
    <div className="min-h-screen bg-zinc-950 text-white flex flex-col items-center px-4 py-8 gap-6">
      <div className="w-full max-w-4xl flex flex-col gap-4">
        <button
          onClick={() => navigate('/')}
          className="text-sm text-zinc-400 hover:text-white transition-colors self-start"
        >
          ← Back
        </button>

        {loading && <p className="text-zinc-400 text-sm">Loading...</p>}
        {error && <p className="text-red-400 text-sm">Failed to load video.</p>}

        {metadata && playBackUrl && (
          <>
            <div className="w-full aspect-video bg-zinc-900 rounded-xl overflow-hidden border border-zinc-800">
              <video controls className="w-full h-full object-contain">
                <source src={playBackUrl} type="video/mp4" />
                Your browser does not support the video tag.
              </video>
            </div>

            <div className="flex flex-col gap-1">
              <h1 className="text-lg font-semibold">{metadata.title}</h1>
              <p className="text-sm text-zinc-400">13203 views</p>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
