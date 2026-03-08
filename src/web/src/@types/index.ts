export type FeedContent = {
  videos: VideoContent[]
}

export type WatchParams = {
  d: string
}

type Base = { id: string; title: string }

export type ResourceType = {
  name: string
  display: string
  id: number
  status: string
}
export type VideoContent = Base & {
  type: 'videos'
  status: string
  tags: string[]
  processed_key: string
  views: number
}

export type ArticleContent = Base & {
  type: 'article'
  excerpt: string
  readTime: string
}

export type SnippetContent = Base & {
  type: 'snippet'
  preview: string
  language: string
}

export type GuideContent = Base & {
  type: 'guide'
  steps: string[]
}

export type Content = VideoContent | ArticleContent | SnippetContent | GuideContent
