use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a video frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub timestamp: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Shared application state
pub struct AppState {
    pub active_device: Arc<Mutex<Option<String>>>,
    pub is_mirroring: Arc<Mutex<bool>>,
    pub is_recording: Arc<Mutex<bool>>,
    pub recording_path: Arc<Mutex<Option<String>>>,
    pub frame_tx: broadcast::Sender<Frame>,
    _frame_rx: broadcast::Receiver<Frame>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(32);
        Self {
            active_device: Arc::new(Mutex::new(None)),
            is_mirroring: Arc::new(Mutex::new(false)),
            is_recording: Arc::new(Mutex::new(false)),
            recording_path: Arc::new(Mutex::new(None)),
            frame_tx: tx,
            _frame_rx: rx,
        }
    }

    pub fn get_frame_rx(&self) -> broadcast::Receiver<Frame> {
        self.frame_tx.subscribe()
    }
}
