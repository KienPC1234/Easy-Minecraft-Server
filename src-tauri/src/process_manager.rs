use std::collections::HashMap;
use std::sync::Arc;
use tokio::{io::{BufReader, AsyncBufReadExt}, process::Command as TokioCommand};
use tauri::async_runtime::spawn;
use tauri::{Emitter, Window};
use std::process::Stdio;
use tokio::sync::Mutex; 

#[derive(Clone)]
pub struct ProcessHandle {
    processes: Arc<Mutex<HashMap<i32, tokio::process::Child>>>,
}

impl ProcessHandle {

    pub fn new() -> Self {
        ProcessHandle {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn stop(&self, id: i32) -> Result<(), String> {
        let mut processes = self.processes.lock().await; // Sử dụng await để lấy MutexGuard
        if let Some(mut child) = processes.remove(&id) {
            child.kill().await.map_err(|e| format!("Failed to kill process {}: {}", id, e))?;
            child.wait().await.map_err(|e| format!("Failed to wait for process {}: {}", id, e))?;
        }
        Ok(())
    }

    pub fn is_running(&self, id: i32) -> bool {
        let processes = self.processes.blocking_lock(); // Sử dụng blocking_lock nếu không async
        processes.contains_key(&id)
    }

    pub fn start(&self, id: i32, args: Vec<String>, window: Window) {
        let processes_arc = self.processes.clone();
        let window_clone = window.clone();

        spawn(async move {
            let mut command = TokioCommand::new("tools/zrok.exe");
            command.args(&args);
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
            command.stdin(Stdio::null());

            #[cfg(windows)]
            {
                command.creation_flags(0x08000000); // CREATE_NO_WINDOW
            }

            match command.spawn() {
                Ok(mut child_proc) => {
                    let stdout = child_proc.stdout.take().unwrap(); // Sử dụng unwrap
                    let stderr = child_proc.stderr.take().unwrap(); // Sử dụng unwrap
                    let stdout_reader = BufReader::new(stdout);
                    let stderr_reader = BufReader::new(stderr);
                    let mut stdout_lines = stdout_reader.lines();
                    let mut stderr_lines = stderr_reader.lines();

                    {
                        let mut processes = processes_arc.lock().await; // Sử dụng await để lấy MutexGuard
                        processes.insert(id, child_proc);
                    }

                    // Xử lý stdout
                    let stdout_window = window.clone();
                    spawn(async move {
                        while let Ok(Some(line)) = stdout_lines.next_line().await {
                            if let Err(e) = stdout_window.emit("zrok-output", format!("ID {}: {}", id, line)) {
                                eprintln!("Failed to emit stdout for ID {}: {}", id, e);
                            }
                        }
                    });

                    // Xử lý stderr
                    let stderr_window = window_clone.clone();
                    spawn(async move {
                        while let Ok(Some(line)) = stderr_lines.next_line().await {
                            if let Err(e) = stderr_window.emit("zrok-output", format!("ID {} ERROR: {}", id, line)) {
                                eprintln!("Failed to emit stderr for ID {}: {}", id, e);
                            }
                        }
                    });

                    // Spawn task riêng để chờ tiến trình kết thúc
                    let processes_arc_clone = processes_arc.clone();
                    spawn(async move {
                        let child = {
                            let mut processes = processes_arc_clone.lock().await; // Sử dụng await để lấy MutexGuard
                            processes.remove(&id)
                        };
                        if let Some(mut child) = child {
                            if let Ok(status) = child.wait().await {
                                println!("Process {} exited with status: {}", id, status);
                            }
                        }
                    });
                }
                Err(e) => {
                    if let Err(emit_err) = window_clone.emit("zrok-output", format!("Failed to start zrok.exe for ID {}: {}", id, e)) {
                        eprintln!("Failed to emit error for ID {}: {}", id, emit_err);
                    }
                }
            }
        });
    }
}