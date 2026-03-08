import type { Content } from '../@types'
import { VideoCard } from './video-card'

// function formatViews(n: number) {
//   if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`
//   if (n >= 1_000) return `${(n / 1_000).toFixed(0)}K`
//   return n.toString()
// }

// function ArticleCard({ item }: { item: Extract<Content, { type: 'article' }> }) {
//   return (
//     <button className="flex flex-col gap-3 text-left group p-4 bg-zinc-900 rounded-lg border border-zinc-800 hover:border-zinc-600 transition-colors">
//       <div className="flex items-center justify-between">
//         <span className="font-mono text-xs text-zinc-500 border border-zinc-700 px-1.5 py-0.5 rounded">
//           article
//         </span>
//         <span className="font-mono text-xs text-[#00a650]">#{item.tag}</span>
//       </div>
//       <p className="text-sm font-medium leading-snug group-hover:text-zinc-300 transition-colors">
//         {item.title}
//       </p>
//       <p className="text-xs text-zinc-500 leading-relaxed line-clamp-2">{item.excerpt}</p>
//       <p className="font-mono text-xs text-zinc-600">{item.readTime} read</p>
//     </button>
//   )
// }

// function SnippetCard({ item }: { item: Extract<Content, { type: 'snippet' }> }) {
//   return (
//     <button className="flex flex-col gap-3 text-left group p-4 bg-zinc-900 rounded-lg border border-zinc-800 hover:border-zinc-600 transition-colors">
//       <div className="flex items-center justify-between">
//         <span className="font-mono text-xs text-zinc-500 border border-zinc-700 px-1.5 py-0.5 rounded">
//           snippet
//         </span>
//         <span className="font-mono text-xs text-[#00a650]">#{item.tag}</span>
//       </div>
//       <p className="text-sm font-medium leading-snug group-hover:text-zinc-300 transition-colors">
//         {item.title}
//       </p>
//       <pre className="text-xs font-mono text-zinc-400 bg-zinc-950 rounded p-3 overflow-hidden line-clamp-3 border border-zinc-800">
//         {item.preview}
//       </pre>
//       <p className="font-mono text-xs text-zinc-600">{item.language}</p>
//     </button>
//   )
// }

// function GuideCard({ item }: { item: Extract<Content, { type: 'guide' }> }) {
//   return (
//     <button className="flex flex-col gap-3 text-left group p-4 bg-zinc-900 rounded-lg border border-zinc-800 hover:border-zinc-600 transition-colors">
//       <div className="flex items-center justify-between">
//         <span className="font-mono text-xs text-zinc-500 border border-zinc-700 px-1.5 py-0.5 rounded">
//           guide
//         </span>
//         <span className="font-mono text-xs text-[#00a650]">#{item.tag}</span>
//       </div>
//       <p className="text-sm font-medium leading-snug group-hover:text-zinc-300 transition-colors">
//         {item.title}
//       </p>
//       <ul className="flex flex-col gap-1">
//         {item.steps.slice(0, 3).map((step, i) => (
//           <li key={i} className="flex gap-2 text-xs text-zinc-500">
//             <span className="font-mono text-[#00a650] shrink-0">{i + 1}.</span>
//             <span className="line-clamp-1">{step}</span>
//           </li>
//         ))}
//         {item.steps.length > 3 && (
//           <li className="font-mono text-xs text-zinc-600">+{item.steps.length - 3} more steps</li>
//         )}
//       </ul>
//     </button>
//   )
// }

export function ContentCard({ item }: { item: Content }) {
  if (item.type === 'videos') return <VideoCard item={item} />
  // if (item.type === 'article') return <ArticleCard item={item} />
  // if (item.type === 'snippet') return <SnippetCard item={item} />
  // if (item.type === 'guide') return <GuideCard item={item} />
  return null
}
