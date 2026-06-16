use std::io::Read;
use std::process::Child;

use tokio::sync::broadcast;
use crate::adb::device::{adb_shell, adb_exec_out};
use crate::video::Frame;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ScreenStream {
    child: Option<Child>,
    serial: String,
    width: i32,
    height: i32,
    fps: i32,
}

impl ScreenStream {
    pub fn start(serial: &str) -> Result<Self, String> {
        let (width, _height) = get_display_info(serial);
        let fps = 30;

        let child = adb_exec_out(serial, &[
            "--profile", "normal",
            "--time-limit", "3600",
            "-",
        ])?;

        Ok(ScreenStream {
            child: Some(child),
            serial: serial.to_string(),
            width,
            height: 1920,
            fps,
        })
    }

    pub async fn read_frames(
        &mut self,
        frame_tx: &broadcast::Sender<Frame>,
    ) -> Result<(), String> {
        let child = self.child.as_mut()
            .ok_or_else(|| "Stream not started".to_string())?;

        let stdout = child.stdout.take()
            .ok_or_else(|| "No stdout".to_string())?;

        let mut buffer = Vec::new();
        let mut raw = std::io::BufReader::new(stdout);

        loop {
            let mut chunk = vec![0u8; 8192];
            match raw.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => {
                    chunk.truncate(n);
                    buffer.extend_from_slice(&chunk);

                    self.process_buffer(&buffer, frame_tx)?;
                    buffer.clear();
                }
                Err(e) => {
                    tracing::warn!("Read error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    fn process_buffer(
        &self,
        buffer: &[u8],
        frame_tx: &broadcast::Sender<Frame>,
    ) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let frame = Frame {
            timestamp,
            width: self.width as u32,
            height: self.height as u32,
            data: buffer.to_vec(),
        };

        if frame_tx.send(frame).is_err() {
            tracing::warn!("No frame receivers");
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.child.take() {
            child.kill().map_err(|e| format!("Failed to kill stream: {}", e))?;
            let _ = child.wait();
        }
        Ok(())
    }
}

impl Drop for ScreenStream {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn get_display_info(serial: &str) -> (i32, i32) {
    let output = adb_shell(serial, &["wm", "size"])
        .ok()
        .and_then(|output| {
            output.lines()
                .flat_map(|line| line.split(':'))
                .nth(1)
                .and_then(|s| s.trim().parse().ok())
        })
        .unwrap_or(1080);

    (output, 1920)
}
