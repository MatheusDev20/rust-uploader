import { useQuery } from '@tanstack/react-query'
import { GET } from '../../api/handlers/GET'
import type { VideoContent } from '../../@types'

export const useVideosList = () => {
  const { data } = useQuery<{ data: VideoContent[] }>({
    queryKey: ['feed-videos'],
    queryFn: () => GET({ path: 'videos' }),
  })

  return { videos: data }
}
