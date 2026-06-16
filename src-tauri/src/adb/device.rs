use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub serial: String,
    pub state: String,
    pub model: String,
    pub transport_id: Option<String>,
}

#[tauri::command]
pub fn list_devices() -> Result<Vec<Device>, String> {
    let adb_path = which::which("adb")
        .map_err(|_| "ADB not found. Install Android SDK Platform-Tools.".to_string())?;

    let output = Command::new(&adb_path)
        .args(["devices", "-l"])
        .output()
        .map_err(|e| format!("ADB command failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "device" {
            let serial = parts[0].to_string();

            let model = parts
                .iter()
                .skip(2)
                .find(|p| p.starts_with("model:"))
                .map(|p| p.strip_prefix("model:").unwrap_or("").to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let transport_id = parts
                .iter()
                .skip(2)
                .find(|p| p.starts_with("transport-id:"))
                .and_then(|p| p.strip_prefix("transport-id:"))
                .map(String::from);

            devices.push(Device {
                serial,
                state: "device".to_string(),
                model,
                transport_id,
            });
        }
    }

    Ok(devices)
}

#[tauri::command]
pub fn connect_wifi(address: String) -> Result<String, String> {
    let adb_path = which::which("adb").map_err(|_| "ADB not found".to_string())?;

    let output = Command::new(&adb_path)
        .args(["connect", &address])
        .output()
        .map_err(|e| format!("ADB connect failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if stdout.contains("connected") || stderr.contains("connected") {
        Ok(format!("Connected to {}", address))
    } else {
        Err(format!(
            "Failed to connect to {}: {}",
            address,
            stdout.trim()
        ))
    }
}

#[tauri::command]
pub fn disconnect_device(serial: String) -> Result<(), String> {
    let adb_path = which::which("adb").map_err(|_| "ADB not found".to_string())?;

    let _ = Command::new(&adb_path)
        .args(["disconnect", &serial])
        .output();

    Ok(())
}

pub fn adb_shell(serial: &str, args: &[&str]) -> Result<String, String> {
    let adb_path = which::which("adb").map_err(|_| "ADB not found".to_string())?;

    let mut cmd = vec!["-s", serial, "shell"];
    cmd.extend(args.iter().map(|s| *s));

    let output = Command::new(&adb_path)
        .args(&cmd)
        .output()
        .map_err(|e| format!("ADB shell failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr)
    }
}

pub fn adb_exec_out(serial: &str, args: &[&str]) -> Result<std::process::Child, String> {
    let adb_path = which::which("adb").map_err(|_| "ADB not found".to_string())?;

    let mut cmd = vec!["-s", serial, "exec-out", "screenrecord"];
    cmd.extend(args.iter().map(|s| *s));

    let child = Command::new(&adb_path)
        .args(&cmd)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start screenrecord: {}", e))?;

    Ok(child)
}
