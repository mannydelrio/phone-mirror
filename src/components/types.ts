export interface Device {
  serial: string
  state: string
  model: string
  transport_id?: string
}

export interface Status {
  mirroring: boolean
  recording: boolean
  fps: number
  error: string | null
}

export interface MirrorSettings {
  fps: number
  maxSize: number
  bitrate: number
  orientation: 'portrait' | 'landscape' | 'auto'
}
