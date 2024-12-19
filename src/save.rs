use crate::structs::Controller;
use dirs_next as dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::path::PathBuf;
use std::sync::Arc;

const MAX_LOG_SIZE: usize = 1024 * 1024; // 1 MB
const STATE_FILE_NAME: &str = "state.json";

// TODO: Refactor out or further integrate
#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub lightbar_enabled: bool,
    pub battery_percentage: f64,
    pub playerleds: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            lightbar_enabled: false,
            battery_percentage: 0.0,
            playerleds: 1,
        }
    }
}

pub struct AppPaths {
    pub config: PathBuf,
    pub logs: PathBuf,
    pub profiles: PathBuf,
    pub log_file: PathBuf,
}

impl AppPaths {
    pub fn new() -> Self {
        let data_local_path = dirs::data_local_dir()
            .unwrap_or_else(|| {
                let fallback = dirs::home_dir()
                    .expect("Failed to find home directory.")
                    .join(".local/share");
                println!(
                    "$XDG_DATA_HOME not set, falling back to {}",
                    fallback.display()
                );
                fallback
            })
            .join("dualsensectl-gui");

        let logs_path = data_local_path.join("logs");
        if !logs_path.exists() {
            println!(
                "Attempting to create logs directory at: {}",
                logs_path.display()
            );
            fs::create_dir_all(&logs_path).expect("Failed to create logs directory");
        }

        let profiles_path = data_local_path.join("profiles");
        if !profiles_path.exists() {
            println!(
                "Attempting to create profiles directory at: {}",
                profiles_path.display()
            );
            fs::create_dir_all(&profiles_path).expect("Failed to create profiles directory");
        }

        let config_path = dirs::config_dir()
            .expect("Failed to determine config directory")
            .join("dualsensectl-gui");

        if !config_path.exists() {
            println!(
                "Attempting to create config directory at: {}",
                config_path.display()
            );
            fs::create_dir_all(&config_path).expect("Failed to create config directory");
        }

        let log_file_path = logs_path.join("dualsensectl.log");

        println!("Config path is: {}", config_path.display());
        println!("Log file path is: {}", log_file_path.display());
        println!("Profiles path is: {}", profiles_path.display());

        AppPaths {
            config: config_path,
            logs: logs_path,
            profiles: profiles_path,
            log_file: log_file_path,
        }
    }
}

pub fn save_state(controller: &Controller, app_paths: &Arc<AppPaths>) -> io::Result<()> {
    let state_file = app_paths.config.join(STATE_FILE_NAME);

    eprintln!("Saving controller state: {:?}", controller);
    let json = serde_json::to_string_pretty(controller)?;
    let mut file = fs::File::create(state_file)?;
    file.write_all(json.as_bytes())?;
    eprintln!("Controller state saved.");
    Ok(())
}

pub fn load_state(app_paths: &Arc<AppPaths>) -> Controller {
    let state_file = app_paths.config.join(STATE_FILE_NAME);

    if let Ok(json) = fs::read_to_string(&state_file) {
        if let Ok(state) = serde_json::from_str::<Controller>(&json) {
            eprintln!("Loaded state: {:?}", state);
            return state;
        } else {
            eprintln!("Failed to deserialize {}", state_file.display());
        }
    } else {
        eprintln!("{} not found, using default state", state_file.display());
    }

    Controller::default()
}

pub fn truncate_log(log_path: &std::path::Path) {
    if let Ok(mut file) = File::open(log_path) {
        let metadata = file.metadata().expect("Failed to get file metadata");
        if metadata.len() as usize > MAX_LOG_SIZE {
            println!(
                "Truncating log file as it exceeds the max size of {} bytes.",
                MAX_LOG_SIZE
            );

            let mut buffer = Vec::with_capacity(MAX_LOG_SIZE / 2);
            file.seek(std::io::SeekFrom::End(-(MAX_LOG_SIZE as i64) / 2))
                .expect("Failed to seek in log file");
            file.read_to_end(&mut buffer)
                .expect("Failed to read log file");

            let mut truncated_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(log_path)
                .expect("Failed to truncate log file");
            truncated_file
                .write_all(&buffer)
                .expect("Failed to write truncated log file");
        }
    }
}
