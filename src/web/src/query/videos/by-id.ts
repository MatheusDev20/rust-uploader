import { useQuery } from '@tanstack/react-query'
import { GET } from '../../api/handlers/GET'
import type { VideoContent } from '../../@types'

export type HookOutput = {
  metadata: VideoContent | null
  playBackUrl: string | null
  error: Error | null
  loading: boolean
}

export function useVideo(id: string): HookOutput {
  const {
    data: metadata,
    error,
    isLoading,
  } = useQuery({
    queryKey: ['videos', id],
    queryFn: () => GET<VideoContent>({ path: `videos/${id}` }),
    enabled: Boolean(id),
  })

  const { data: preSignedPayload } = useQuery({
    queryKey: ['pre-signed-url', id],
    queryFn: () => GET<{ url: string }>({ path: `videos/${id}/stream` }),
    enabled: Boolean(id) && !isLoading && !error && !!metadata,
  })

  const preSignedUrl = preSignedPayload?.url ?? null

  return {
    metadata: metadata ?? null,
    playBackUrl: preSignedUrl,
    error: error,
    loading: isLoading,
  }
}
