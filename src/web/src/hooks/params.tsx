import { useCallback, useMemo } from 'react'
import { useSearchParams } from 'react-router-dom'

type ParamValue = string | number | boolean | null | undefined

type SetParamsOptions = {
  replace?: boolean
}

export function useQueryParams<T>() {
  const [searchParams, setSearchParams] = useSearchParams()

  const paramsObject = useMemo(() => {
    return Object.fromEntries(searchParams.entries())
  }, [searchParams])

  const getParam = useCallback(
    (key: string) => {
      return searchParams.get(key)
    },
    [searchParams]
  )

  const getNumberParam = useCallback(
    (key: string, defaultValue?: number) => {
      const value = searchParams.get(key)
      if (value == null) return defaultValue
      const parsed = Number(value)
      return Number.isNaN(parsed) ? defaultValue : parsed
    },
    [searchParams]
  )

  const getBooleanParam = useCallback(
    (key: string, defaultValue?: boolean) => {
      const value = searchParams.get(key)
      if (value == null) return defaultValue
      return value === 'true'
    },
    [searchParams]
  )

  const setParam = useCallback(
    (key: string, value: ParamValue, options?: SetParamsOptions) => {
      const next = new URLSearchParams(searchParams)

      if (value == null || value === '') {
        next.delete(key)
      } else {
        next.set(key, String(value))
      }

      setSearchParams(next, { replace: options?.replace })
    },
    [searchParams, setSearchParams]
  )

  const setManyParams = useCallback(
    (updates: Record<string, ParamValue>, options?: SetParamsOptions) => {
      const next = new URLSearchParams(searchParams)

      for (const [key, value] of Object.entries(updates)) {
        if (value == null || value === '') {
          next.delete(key)
        } else {
          next.set(key, String(value))
        }
      }

      setSearchParams(next, { replace: options?.replace })
    },
    [searchParams, setSearchParams]
  )

  const removeParam = useCallback(
    (key: string, options?: SetParamsOptions) => {
      const next = new URLSearchParams(searchParams)
      next.delete(key)
      setSearchParams(next, { replace: options?.replace })
    },
    [searchParams, setSearchParams]
  )

  const clearParams = useCallback(
    (options?: SetParamsOptions) => {
      setSearchParams({}, { replace: options?.replace })
    },
    [setSearchParams]
  )

  return {
    params: paramsObject as T,
    rawSearchParams: searchParams,
    getParam,
    getNumberParam,
    getBooleanParam,
    setParam,
    setManyParams,
    removeParam,
    clearParams,
  }
}
