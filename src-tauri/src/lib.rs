use std::time::Duration;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

mod commands; 
use commands::greet; 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let sidecar = app.shell().sidecar("yuka-server").unwrap();
            let (_rx, _child) = sidecar
                .env("YUKA_SECRET", "your-32-byte-secret-here-change-me")
                .spawn()
                .expect("failed to start yuka server");

            // Get the main window handle (assuming label is "main")
            let window = app.get_webview_window("main").unwrap();

            // Poll port 8080 in a background thread, navigate after success
            std::thread::spawn(move || {
                for _ in 0..100 { // Up to 10 seconds
                    if std::net::TcpStream::connect("127.0.0.1:8080").is_ok() {
                        let _ = window.navigate("http://localhost:8080".parse().unwrap());
                        return;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                eprintln!("Yuka server did not start listening within 10 seconds");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
