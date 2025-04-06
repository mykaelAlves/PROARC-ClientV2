// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn tcp_client() -> Result<String, String> {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use tauri::Manager;

    // Attempt to connect to the server at localhost:9999
    let mut stream = TcpStream::connect("127.0.0.1:9999")
        .map_err(|e| format!("Failed to connect: {}", e))?;

    // Send a message to the server
    stream
        .write_all(b"Hello from Tauri TCP client!")
        .map_err(|e| format!("Failed to send data: {}", e))?;

    // Buffer for response
    let mut buffer = [0; 1024];
    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    Ok(response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, tcp_client])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
