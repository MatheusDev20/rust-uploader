import api from '../client'

export type Payload = {
  path: string
  headers?: Record<string, string>
}

export async function GET<T>(payload: Payload) {
  const axiosResponse = await api.get<{ data: T }>(payload.path, {
    headers: payload.headers,
  })

  const { data } = axiosResponse

  return data.data
}
