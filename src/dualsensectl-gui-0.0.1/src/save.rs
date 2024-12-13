use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

use crate::structs::Controller;

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

pub fn save_state(controller: &Controller) -> io::Result<()> {
    eprintln!("Saving controller state: {:?}", controller);
    let json = serde_json::to_string_pretty(controller)?;
    let mut file = fs::File::create(STATE_FILE)?;
    file.write_all(json.as_bytes())?;
    eprintln!("Controller state saved to {}", STATE_FILE);
    Ok(())
}

pub fn load_state() -> Controller {
    if let Ok(json) = fs::read_to_string(STATE_FILE) {
        if let Ok(state) = serde_json::from_str::<Controller>(&json) {
            eprintln!("Loaded state: {:?}", state);
            return state;
        } else {
            eprintln!("Failed to deserialize state.json");
        }
    } else {
        eprintln!("state.json not found, using default state");
    }
    Controller::default()
}
