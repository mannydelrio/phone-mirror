import { useState } from 'react'
import { DeviceList, ScreenMirror, ControlPanel } from './components'
import { useDevices, useMirror } from './hooks'
import type { Status } from './components/types'
import './App.css'

function App() {
  const { devices, loading, refreshDevices } = useDevices()
  const mirror = useMirror()
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null)

  const handleDeviceSelect = (serial: string) => {
    setSelectedDevice(serial)
  }

  const handleStatusChange = (updates: Partial<Status>) => {
    console.log('Status update:', updates)
  }

  const handleMirrorToggle = async () => {
    if (!selectedDevice) return

    try {
      if (mirror.isMirroring) {
        await mirror.stopMirror()
      } else {
        await mirror.startMirror(selectedDevice)
      }
    } catch (err) {
      const error = err instanceof Error ? err.message : String(err)
      handleStatusChange({ error, mirroring: false })
    }
  }

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-title">
          <span className="title-icon">📱</span>
          <h1>PhoneMirror</h1>
        </div>
        <div className="app-actions">
          <button onClick={refreshDevices} disabled={loading} title="Refresh devices">
            🔄
          </button>
        </div>
      </header>
      <div className="app-main">
        <aside className="app-sidebar">
          <div className="sidebar-section">
            <h3>Devices</h3>
            <DeviceList
              devices={devices}
              selected={selectedDevice}
              onSelect={handleDeviceSelect}
            />
          </div>
          <div className="sidebar-section">
            <ControlPanel
              device={selectedDevice}
              isMirroring={mirror.isMirroring}
              onMirrorToggle={handleMirrorToggle}
            />
          </div>
        </aside>
        <main className="app-content">
          <ScreenMirror onStatusChange={handleStatusChange} />
        </main>
      </div>
    </div>
  )
}

export default App
