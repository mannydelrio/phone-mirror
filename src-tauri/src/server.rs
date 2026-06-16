use tokio::net::TcpListener;

#[tauri::command]
pub async fn start_web_server(port: Option<u16>) -> Result<String, String> {
    let port = port.unwrap_or(8080);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| format!("Failed to bind to port {}: {}", port, e))?;

    let _addr = listener.local_addr()
        .map_err(|e| format!("Failed to get address: {}", e))?;

    // Spawn server in background
    tokio::spawn(async move {
        // TODO: Implement actual HTTP/WebSocket server
        let _ = listener;
    });

    Ok(format!("http://localhost:{}", port))
}

#[tauri::command]
pub async fn stop_web_server() -> Result<(), String> {
    Ok(())
}
