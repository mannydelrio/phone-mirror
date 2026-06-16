import React from 'react'
import type { Device as DeviceType } from './types'

interface DeviceListProps {
  devices: DeviceType[]
  selected: string | null
  onSelect: (serial: string) => void
}

export const DeviceList: React.FC<DeviceListProps> = ({ devices, selected, onSelect }) => {
  if (devices.length === 0) {
    return (
      <div style={{ color: '#999', fontStyle: 'italic' }}>
        No devices found. Connect your device via USB or Wi-Fi.
      </div>
    )
  }

  return (
    <div>
      {devices.map(device => (
        <div
          key={device.serial}
          className={`device-item ${selected === device.serial ? 'selected' : ''}`}
          onClick={() => onSelect(device.serial)}
        >
          <span className="device-icon">📱</span>
          <div className="device-info">
            <div className="device-name">{device.model || 'Unknown'}</div>
            <div className="device-state">
              {device.state} • {device.serial.split(':')[0]}
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}
