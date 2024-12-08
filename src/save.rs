use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

const STATE_FILE: &str = "state.json";

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

pub fn save_state(state: &AppState) -> io::Result<()> {
    eprintln!("Saving state: {:?}", state);
    let json = serde_json::to_string_pretty(state)?;
    let mut file = fs::File::create(STATE_FILE)?;
    file.write_all(json.as_bytes())?;
    eprintln!("State saved to {}", STATE_FILE);
    Ok(())
}

pub fn load_state() -> AppState {
    if let Ok(json) = fs::read_to_string(STATE_FILE) {
        if let Ok(state) = serde_json::from_str::<AppState>(&json) {
            eprintln!("Loaded state: {:?}", state);
            return state;
        } else {
            eprintln!("Failed to deserialize state.json");
        }
    } else {
        eprintln!("state.json not found, using default state");
    }
    AppState::default()
}
