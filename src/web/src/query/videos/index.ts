import { useQuery } from "@tanstack/react-query";
import type { Video } from "../../@types";
import { GET } from "../../api/handlers/GET";


export type HookOutput = {
  metadata: Video | null;
  playBackUrl: string | null;
  error: Error | null;
  loading: boolean;
}

export function useVideo(id: string): HookOutput {

  const { data: metadata, error, isLoading } =  useQuery({
    queryKey: ['videos', id],
    queryFn: () => GET<Video>({ path: `videos/${id}`}),
    enabled: Boolean(id),
  })


  const { data: preSignedPayload } = useQuery({
    queryKey: ['pre-signed-url', id],
    queryFn: () => GET<{ url: string }>({ path: `videos/${id}/stream` }),
    enabled: Boolean(id) && !isLoading && !error && !!metadata
   })

   const preSignedUrl = preSignedPayload?.url ?? null

   return {
    metadata: metadata ?? null,
    playBackUrl: preSignedUrl,
    error: error,
    loading: isLoading
   }

}