import type { Content } from '../@types'
import { useVideosList } from '../query/videos/list-all'

type FeedOutput = {
  content: Content[]
  loading: boolean
  error: Error | null
}
export const useFeed = (): FeedOutput => {
  const { videos } = useVideosList()

  const videosList = videos
    ? videos.data.map((video) => ({ ...video, type: 'videos' as const }))
    : []

  const feed = [...videosList]

  return { content: feed, loading: false, error: null }
}
