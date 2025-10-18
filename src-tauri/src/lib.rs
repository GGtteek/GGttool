use std::process::{Command, Stdio};
use std::env;

#[tauri::command]
fn run_mas_script() -> Result<String, String> {
    // Build full path to the script
    let script_path = env::current_dir()
        .map_err(|e| e.to_string())?
        .join("..")
        .join("Activation_Script.cmd");

    if !script_path.exists() {
        return Err(format!("❌ Script not found at: {}", script_path.display()));
    }

    // Execute the script using cmd
    let output = Command::new("cmd")
        .args(["/C", script_path.to_str().ok_or("Invalid path")?])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(format!("✅ Success:\n{}", stdout))
    } else {
        Err(format!("⚠️ Error:\n{}\n{}", stdout, stderr))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_mas_script])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}