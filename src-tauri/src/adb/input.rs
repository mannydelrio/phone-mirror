use super::device::adb_shell;

#[tauri::command]
pub fn tap(serial: String, x: i32, y: i32) -> Result<(), String> {
    adb_shell(
        &serial,
        &[
            "input",
            "tap",
            &x.to_string(),
            &y.to_string(),
        ],
    )
    .map(|_| ())
}

#[tauri::command]
pub fn swipe(
    serial: String,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    duration: Option<i32>,
) -> Result<(), String> {
    let duration = duration.unwrap_or(100);
    adb_shell(
        &serial,
        &[
            "input",
            "swipe",
            &x1.to_string(),
            &y1.to_string(),
            &x2.to_string(),
            &y2.to_string(),
            &duration.to_string(),
        ],
    )
    .map(|_| ())
}

#[tauri::command]
pub fn key(serial: String, key_code: String) -> Result<(), String> {
    adb_shell(&serial, &["input", "keyevent", &key_code]).map(|_| ())
}

#[tauri::command]
pub fn text(serial: String, input_text: String) -> Result<(), String> {
    let escaped = input_text
        .replace(" ", "%s")
        .replace("&", "%26")
        .replace("<", "%3C")
        .replace(">", "%3E")
        .replace("|", "%7C")
        .replace(";", "%3B")
        .replace("!", "%21")
        .replace("$", "%24")
        .replace("'", "%27")
        .replace("#", "%23")
        .replace("(", "%28")
        .replace(")", "%29")
        .replace("`", "%60")
        .replace("\\", "%5C");

    adb_shell(&serial, &["input", "text", &escaped]).map(|_| ())
}

#[tauri::command]
pub fn home(serial: String) -> Result<(), String> {
    adb_shell(&serial, &["input", "keyevent", "KEYCODE_HOME"]).map(|_| ())
}

#[tauri::command]
pub fn back(serial: String) -> Result<(), String> {
    adb_shell(&serial, &["input", "keyevent", "KEYCODE_BACK"]).map(|_| ())
}

#[tauri::command]
pub fn recent_apps(serial: String) -> Result<(), String> {
    adb_shell(&serial, &["input", "keyevent", "KEYCODE_APP_SWITCH"]).map(|_| ())
}

#[tauri::command]
pub fn get_display_info(serial: String) -> Result<String, String> {
    let output = adb_shell(&serial, &["wm", "size"])?;
    Ok(output.trim().to_string())
}
