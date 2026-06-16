import { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { Device } from '../components/types'

export function useDevices() {
  const [devices, setDevices] = useState<Device[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const refreshDevices = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const result = await invoke<Device[]>('get_devices')
      setDevices(Array.isArray(result) ? result : [])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to list devices')
      setDevices([])
    } finally {
      setLoading(false)
    }
  }, [])

  // Auto-refresh every 10 seconds
  useEffect(() => {
    refreshDevices()
    const interval = setInterval(refreshDevices, 10000)
    return () => clearInterval(interval)
  }, [refreshDevices])

  return {
    devices,
    loading,
    error,
    refreshDevices,
  }
}
