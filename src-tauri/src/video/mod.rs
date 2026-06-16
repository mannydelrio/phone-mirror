pub mod stream;
pub mod recorder;
pub mod decoder;

use crate::state::{AppState, Frame};
use tauri::State;

#[tauri::command]
pub async fn start_mirroring(
    serial: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Update state
    {
        let mut active = state.active_device.lock().await;
        *active = Some(serial.clone());
    }

    let mut is_mirroring = state.is_mirroring.lock().await;
    if *is_mirroring {
        return Err("Already mirroring another device".to_string());
    }
    *is_mirroring = true;
    drop(is_mirroring);

    // Start the screen stream
    let mut screen_stream = stream::ScreenStream::start(&serial)?;

    // Spawn async task to read frames
    let frame_tx = state.frame_tx.clone();
    let is_mirroring = state.is_mirroring.clone();
    let active_device = state.active_device.clone();

    tokio::spawn(async move {
        if let Err(e) = screen_stream.read_frames(&frame_tx).await {
            tracing::error!("Stream error: {}", e);
            *is_mirroring.lock().await = false;
            let mut active = active_device.lock().await;
            *active = None;
        }
    });

    Ok(format!("Mirroring device {}", serial))
}

#[tauri::command]
pub async fn stop_mirroring(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut is_mirroring = state.is_mirroring.lock().await;
    *is_mirroring = false;
    drop(is_mirroring);

    let mut active = state.active_device.lock().await;
    *active = None;

    Ok(())
}

#[tauri::command]
pub async fn start_recording(
    path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let is_mirroring = *state.is_mirroring.lock().await;
    if !is_mirroring {
        return Err("Start mirroring first".to_string());
    }

    {
        let mut recording = state.is_recording.lock().await;
        *recording = true;
    }

    {
        let mut rec_path = state.recording_path.lock().await;
        *rec_path = Some(path.clone());
    }

    Ok(format!("Recording to {}", path))
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let mut is_recording = state.is_recording.lock().await;
    *is_recording = false;
    drop(is_recording);

    let path = {
        let mut rec_path = state.recording_path.lock().await;
        rec_path.take()
    };

    Ok(path)
}
