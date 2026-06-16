import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'

export function useMirror() {
  const [isMirroring, setIsMirroring] = useState(false)
  const [isRecording, setIsRecording] = useState(false)
  const [currentDevice, setCurrentDevice] = useState<string | null>(null)

  const startMirror = useCallback(async (serial: string) => {
    try {
      await invoke('start_mirroring', { serial })
      setIsMirroring(true)
      setCurrentDevice(serial)
    } catch (err) {
      setIsMirroring(false)
      setCurrentDevice(null)
      throw err
    }
  }, [])

  const stopMirror = useCallback(async () => {
    try {
      await invoke('stop_mirroring')
      setIsMirroring(false)
      setIsRecording(false)
      setCurrentDevice(null)
    } catch (err) {
      console.error('Failed to stop mirroring:', err)
    }
  }, [])

  const startRecording = useCallback(async (path: string) => {
    try {
      await invoke('start_recording', { path })
      setIsRecording(true)
    } catch (err) {
      throw err
    }
  }, [])

  const stopRecording = useCallback(async () => {
    try {
      const result = await invoke<string | null>('stop_recording')
      setIsRecording(false)
      return result as string | null
    } catch (err) {
      setIsRecording(false)
      throw err
    }
  }, [])

  return {
    isMirroring,
    isRecording,
    currentDevice,
    startMirror,
    stopMirror,
    startRecording,
    stopRecording,
  }
}
