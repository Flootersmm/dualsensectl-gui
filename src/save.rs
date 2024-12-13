use crate::structs::Controller;
use dirs_next as dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const STATE_FILE_NAME: &str = "state.json";

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

// ~/.config/dualsensectl-gui
fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to determine config directory");
    config_dir.join("dualsensectl-gui")
}

pub fn save_state(controller: &Controller) -> io::Result<()> {
    let config_path = get_config_path();

    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }

    let state_file = config_path.join(STATE_FILE_NAME);

    eprintln!("Saving controller state: {:?}", controller);
    let json = serde_json::to_string_pretty(controller)?;
    let mut file = fs::File::create(state_file)?;
    file.write_all(json.as_bytes())?;
    eprintln!("Controller state saved.");
    Ok(())
}

pub fn load_state() -> Controller {
    let config_path = get_config_path();
    let state_file = config_path.join(STATE_FILE_NAME);

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
