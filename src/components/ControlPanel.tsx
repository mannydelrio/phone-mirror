import React, { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface ControlPanelProps {
  device: string | null
  isMirroring: boolean
  onMirrorToggle: () => void
}

export const ControlPanel: React.FC<ControlPanelProps> = ({
  device,
  isMirroring,
  onMirrorToggle,
}) => {
  const [recording, setRecording] = useState(false)
  const [wifiAddress, setWifiAddress] = useState('')
  const [inputText, setInputText] = useState('')

  const handleConnectWifi = async () => {
    try {
      await invoke('connect_wifi_device', { address: wifiAddress })
    } catch (error) {
      console.error('Wi-Fi connect failed:', error)
    }
  }

  const handleRecordingToggle = async () => {
    try {
      if (recording) {
        await invoke('stop_recording')
      } else {
        await invoke('start_recording', { path: '/tmp/recording.mp4' })
      }
      setRecording(!recording)
    } catch (error) {
      console.error('Recording failed:', error)
    }
  }

  const handleKey = async (key: string) => {
    if (!device) return
    try {
      await invoke('device_key', { serial: device, keyCode: key })
    } catch (error) {
      console.error('Key failed:', error)
    }
  }

  const handleTextInput = async () => {
    if (!device || !inputText) return
    try {
      await invoke('device_text', { serial: device, text: inputText })
      setInputText('')
    } catch (error) {
      console.error('Text input failed:', error)
    }
  }

  return (
    <div>
      <div className="control-section">
        <h3>Connection</h3>
        
        <div style={{ marginBottom: '1rem' }}>
          <input
            type="text"
            placeholder="192.168.1.100:5555"
            value={wifiAddress}
            onChange={e => setWifiAddress(e.target.value)}
            style={{
              width: '100%',
              padding: '0.5rem',
              border: '1px solid #e0e0e0',
              borderRadius: '6px',
              fontSize: '0.9rem',
            }}
          />
          <button
            onClick={handleConnectWifi}
            className="control-btn"
            style={{ marginTop: '0.5rem' }}
          >
            Connect via Wi-Fi
          </button>
        </div>
      </div>

      <div className="control-section">
        <h3>Screen</h3>
        <button
          className={`mirror-btn ${isMirroring ? 'stop' : 'start'}`}
          onClick={onMirrorToggle}
          disabled={!device}
        >
          {isMirroring ? '⏹ Stop Mirroring' : '▶ Start Mirroring'}
        </button>
        
        <button
          className="control-btn"
          onClick={handleRecordingToggle}
          disabled={!isMirroring}
        >
          {recording ? '⏹ Stop Recording' : '⏺ Start Recording'}
        </button>
      </div>

      <div className="control-section">
        <h3>Navigation</h3>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_HOME')}>
          🏠 Home
        </button>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_BACK')}>
          ← Back
        </button>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_APP_SWITCH')}>
          ⬜ Recent Apps
        </button>
      </div>

      <div className="control-section">
        <h3>Volume</h3>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_VOLUME_UP')}>
          🔊 Volume Up
        </button>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_VOLUME_DOWN')}>
          🔉 Volume Down
        </button>
        <button className="control-btn" onClick={() => handleKey('KEYCODE_VOLUME_MUTE')}>
          🔇 Mute
        </button>
      </div>

      <div className="control-section">
        <h3>Type Text</h3>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <input
            type="text"
            placeholder="Type here..."
            value={inputText}
            onChange={e => setInputText(e.target.value)}
            onKeyPress={e => e.key === 'Enter' && handleTextInput()}
            style={{
              flex: 1,
              padding: '0.5rem',
              border: '1px solid #e0e0e0',
              borderRadius: '6px',
              fontSize: '0.9rem',
            }}
          />
          <button onClick={handleTextInput}>Send</button>
        </div>
      </div>
    </div>
  )
}
