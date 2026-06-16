import React, { useEffect, useRef, useState } from 'react'
import type { Status } from './types'

interface ScreenMirrorProps {
  onStatusChange: (status: Partial<Status>) => void
}

export const ScreenMirror: React.FC<ScreenMirrorProps> = ({ onStatusChange }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let isMirroring = false
    let animationId: number | null = null
    const canvas = canvasRef.current

    const mirrorScreen = async () => {
      try {
        if (!isMirroring) {
          isMirroring = true
          onStatusChange({ mirroring: true, error: null })
        }

        if (canvas) {
          const ctx = canvas.getContext('2d')
          if (ctx) {
            ctx.fillStyle = '#1a1a1a'
            ctx.fillRect(0, 0, canvas.width, canvas.height)

            ctx.fillStyle = '#ffffff'
            ctx.font = '16px Inter, sans-serif'
            ctx.textAlign = 'center'
            ctx.fillText('Phone screen will appear here', canvas.width / 2, canvas.height / 2 - 20)
            ctx.font = '14px Inter, sans-serif'
            ctx.fillStyle = '#999'
            ctx.fillText('Connect a device and start mirroring', canvas.width / 2, canvas.height / 2 + 10)
          }
        }
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        setError(error)
        onStatusChange({ mirroring: false, error })
        isMirroring = false
      }
    }

    mirrorScreen()
    animationId = window.setInterval(mirrorScreen, 1000 / 30)

    return () => {
      if (animationId) {
        window.clearInterval(animationId)
      }
    }
  }, [onStatusChange])

  return (
    <div className="screen-mirror">
      <div className="mirror-container">
        <canvas
          ref={canvasRef}
          className="mirror-canvas"
          width={360}
          height={640}
        />
        {error && (
          <div className="mirror-error">
            Error: {error}
          </div>
        )}
      </div>
    </div>
  )
}
