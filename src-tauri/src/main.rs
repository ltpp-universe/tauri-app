use std::{
    os::windows::process::ExitStatusExt,
    process::{Command, Output},
};

#[tauri::command]
fn run_command(command: String) -> Result<String, String> {
    let _default_output = Output {
        status: std::process::ExitStatus::from_raw(1),
        stdout: vec![],
        stderr: b"shell run error".to_vec(),
    };
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg(&command)
            .output()
            .unwrap_or(_default_output)
    } else {
        Command::new("sh")
            .arg(&command)
            .output()
            .unwrap_or(_default_output)
    };
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        return Ok(result);
    }
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    Ok(error)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
