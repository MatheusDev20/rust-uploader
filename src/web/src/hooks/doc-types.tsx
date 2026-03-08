import { useQuery } from '@tanstack/react-query'
import { GET } from '../api/handlers/GET'
import { type ResourceType } from '../@types'

export const useResourceTypes = () => {
  const { data, isLoading } = useQuery({
    queryKey: ['resource-types'],
    queryFn: () => GET<ResourceType[]>({ path: 'resource-types' }),
  })

  return {
    resourceTypes: data ?? [],
    isLoading: !data || isLoading,
  }
}
