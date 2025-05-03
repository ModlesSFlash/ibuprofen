// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::{self};
use std::io::{Write};
use std::net::TcpStream;
use std::process::Command;

mod config;
use crate::config::{CONFIG, APP_CONFIG};

#[tauri::command]
fn is_config_missing() -> bool {
    !get_config_path().exists()
}

fn get_config_path() -> std::path::PathBuf {
    let exe_dir = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Executable must be in a valid directory")
        .to_path_buf();

    exe_dir.join("config.toml")
}

#[tauri::command]
fn save_config(config: APP_CONFIG) -> Result<(), String> {
    let toml_string = format!(
        r#"# be mindful of "quotes"

# link: https://developers.google.com/tenor/guides/quickstart#setup
google_api_key = "{google_api_key}"
tenor_api_key = "{tenor_api_key}"

# link: https://programmablesearchengine.google.com/controlpanel/all
google_cx = "{google_cx}"

# match in ibuprofen-listener
resolve_host = "{resolve_host}"
resolve_port = {resolve_port}

# use \\ on Windows
download_dir = "{download_dir}"
"#,
        google_api_key = config.google_api_key,
        tenor_api_key = config.tenor_api_key,
        google_cx = config.google_cx,
        resolve_host = config.resolve_host,
        resolve_port = config.resolve_port,
        download_dir = config.download_dir
    );

    fs::write(get_config_path(), toml_string).map_err(|e| e.to_string())
}

#[tauri::command]
fn parse_config() -> Result<serde_json::Value, String> {
    let config = serde_json::to_value(&*CONFIG).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
fn download_png(url: String, filename: String) -> Result<(), String> {
    let download_dir = &CONFIG.download_dir;
    fs::create_dir_all(download_dir).map_err(|e| e.to_string())?;

    let temp_file = format!("{}{}_temp", download_dir, filename); // no extension
    let final_png = format!("{}{}.png", download_dir, filename);

    // download the image (curl)
    let status = Command::new("curl")
        .args(&["-L", "-o", &temp_file, &url])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Failed to download image".into());
    }

    // convert to PNG (ffmpeg)
    let status = Command::new("ffmpeg")
        .args(&["-y", "-i", &temp_file, &final_png])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Failed to convert image to PNG".into());
    }

    // remove the original file
    fs::remove_file(&temp_file).ok();

    // send png file path to ibuprofen-listener
    let mut stream = TcpStream::connect((&CONFIG.resolve_host[..], CONFIG.resolve_port))
        .map_err(|e| format!("Failed to connect to ibuprofen-listener: {}", e))?;

    writeln!(stream, "{}", final_png)
        .map_err(|e| format!("Failed to write to socket: {}", e))?;

    Ok(())
}

#[tauri::command]
fn download_gif(url: String, filename: String) -> Result<(), String> {
    let download_dir = &CONFIG.download_dir;
    fs::create_dir_all(download_dir).map_err(|e| e.to_string())?;

    let final_gif = format!("{}{}.gif", download_dir, filename);

    let status = Command::new("curl")
        .args(&["-L", "-o", &final_gif, &url])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Failed to download GIF".into());
    }

    let mut stream = TcpStream::connect((&CONFIG.resolve_host[..], CONFIG.resolve_port))
        .map_err(|e| format!("Failed to connect to Resolve listener: {}", e))?;

    writeln!(stream, "{}", final_gif)
        .map_err(|e| format!("Failed to write to socket: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            is_config_missing,
            save_config,
            parse_config,
            download_png,
            download_gif
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
