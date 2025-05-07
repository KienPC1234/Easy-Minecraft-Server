mod process_manager;
use process_manager::ProcessHandle;
use tokio::sync::Mutex; 
use tauri::State;
use sysinfo::System;
use std::net::TcpListener;
use std::collections::HashMap;
use pnet::datalink;

#[tauri::command]
async fn start_zrok(
    id: i32,
    args: Vec<String>,
    handle: State<'_, Mutex<ProcessHandle>>,
    window: tauri::Window,
) -> Result<(), String> {
    let is_running = {
        let manager = handle.lock().await; // Sử dụng await để lấy MutexGuard
        manager.is_running(id)
    };

    if is_running {
        return Err(format!("Process with ID {} is already running", id));
    }

    {
        let manager = handle.lock().await; // Sử dụng await để lấy MutexGuard
        manager.start(id, args, window);
    }
    Ok(())
}

#[tauri::command]
async fn stop_zrok(id: i32, handle: State<'_, Mutex<ProcessHandle>>) -> Result<(), String> {
    let manager = handle.lock().await; // Sử dụng await để lấy MutexGuard
    let result = manager.stop(id).await;
    drop(manager); // Thả lock trước khi trả về kết quả
    result
}

#[tauri::command]
async fn find_java_ports(ports: Option<Vec<u16>>) -> Result<HashMap<String, u16>, String> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut result = HashMap::new();
    // Lấy địa chỉ IPv4 của máy
    let local_ip = datalink::interfaces()
        .into_iter()
        .flat_map(|iface| iface.ips)
        .filter_map(|ip| match ip {
            pnet::ipnetwork::IpNetwork::V4(ipv4) => Some(ipv4.ip().to_string()),
            _ => None,
        })
        .find(|ip| !ip.starts_with("127."))
        .ok_or_else(|| "Failed to get local IPv4 address".to_string())?;

    // Xác định danh sách port để quét
    let ports_to_scan = ports.unwrap_or_else(|| (1024..=65535).collect::<Vec<u16>>());

    for process in system.processes().values() {
        // Kiểm tra tiến trình Java (Minecraft server thường chạy bằng java hoặc javaw)
        let exe_path = process.exe().map(|path| path.to_string_lossy().to_string());
        if let Some(exe) = exe_path {
            if exe.to_lowercase().contains("java") {
                // Kiểm tra các port trong danh sách
                for &port in &ports_to_scan {
                    if let Ok(listener) = TcpListener::bind((local_ip.as_str(), port)) {
                        // Port không được sử dụng, bỏ qua
                        drop(listener);
                        continue;
                    } else {
                        // Port đang được sử dụng, giả sử là bởi tiến trình Java
                        result.insert(local_ip.clone(), port);
                    }
                }
            }
        }
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(ProcessHandle::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_zrok, stop_zrok, find_java_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}