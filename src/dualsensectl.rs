use log::{error, info};
use std::process::Command;

pub fn toggle_lightbar(state: bool) {
    let command = if state {
        "dualsensectl lightbar on"
    } else {
        "dualsensectl lightbar off"
    };

    info!("Executing command: {}", command);

    if let Err(err) = Command::new("sh").arg("-c").arg(command).output() {
        error!("Failed to execute command '{}': {}", command, err);
    }
}

pub fn report_battery() -> String {
    let command = "dualsensectl battery";

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                info!("Command output: {}", stdout.trim());

                stdout
                    .trim()
                    .split_whitespace()
                    .next()
                    .map(|s| format!("{}%", s))
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                error!("Failed to parse command stdout as UTF-8");
                return "Error: Invalid UTF-8 output".to_string();
            }
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
            return format!("Error: {}", err);
        }
    }
}
