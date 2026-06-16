use async_trait::async_trait;
use crate::adb::device::Device;
use crate::video::stream::ScreenStream;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Tap { x: i32, y: i32 },
    Swipe { x1: i32, y1: i32, x2: i32, y2: i32, duration: i32 },
    Key { code: String },
    Text { text: String },
}

#[async_trait]
pub trait DeviceProtocol: Send + Sync {
    /// Discover available devices
    async fn discover_devices() -> Result<Vec<Device>, String>;
    
    /// Start screen streaming from a device
    async fn start_screen_stream(serial: &str) -> Result<ScreenStream, String>;
    
    /// Send input event to device
    async fn send_input(serial: &str, input: InputEvent) -> Result<(), String>;
    
    /// Stop the session
    async fn stop_session(serial: &str) -> Result<(), String>;
    
    /// Get protocol name (for UI)
    fn name(&self) -> &str;
}

/// Android protocol implementation via ADB
pub struct AndroidProtocol;

#[async_trait]
impl DeviceProtocol for AndroidProtocol {
    async fn discover_devices() -> Result<Vec<Device>, String> {
        crate::adb::device::list_devices()
    }

    async fn start_screen_stream(serial: &str) -> Result<ScreenStream, String> {
        ScreenStream::start(serial)
    }

    async fn send_input(serial: &str, input: InputEvent) -> Result<(), String> {
        use crate::adb::input;
        match input {
            InputEvent::Tap { x, y } => input::tap(serial, x, y),
            InputEvent::Swipe { x1, y1, x2, y2, duration } => {
                input::swipe(serial, x1, y1, x2, y2, duration)
            }
            InputEvent::Key { code } => input::key(serial, &code),
            InputEvent::Text { text } => input::text(serial, &text),
        }
    }

    async fn stop_session(serial: &str) -> Result<(), String> {
        crate::adb::device::disconnect_device(serial)
    }

    fn name(&self) -> &str {
        "Android (ADB)"
    }
}

/// iOS protocol implementation via SCRP
/// Uses libimobiledevice for device discovery and pairing
pub struct IosProtocol;

#[async_trait]
impl DeviceProtocol for IosProtocol {
    async fn discover_devices() -> Result<Vec<Device>, String> {
        // TODO: Implement using libimobiledevice
        // Uses ideviceinfo for discovery
        // Requires device pairing first
        Err("iOS support not yet implemented".to_string())
    }

    async fn start_screen_stream(_serial: &str) -> Result<ScreenStream, String> {
        // TODO: Implement SCRP screen capture
        // Reference: https://github.com/michaelwinning/scrcpy-ios
        Err("iOS screen streaming not yet implemented".to_string())
    }

    async fn send_input(_serial: &str, _input: InputEvent) -> Result<(), String> {
        // TODO: Implement SCRP remote input
        Err("iOS input forwarding not yet implemented".to_string())
    }

    async fn stop_session(_serial: &str) -> Result<(), String> {
        // TODO: Implement SCRP session teardown
        Err("iOS session management not yet implemented".to_string())
    }

    fn name(&self) -> &str {
        "iOS (SCRP)"
    }
}
