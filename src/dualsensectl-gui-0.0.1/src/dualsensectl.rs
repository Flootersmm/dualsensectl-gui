use log::{error, info};
use std::process::Command;

use crate::structs::Controller;

pub fn toggle_lightbar(state: bool, controller: &mut Controller) {
    let command = if state {
        format!(
            "dualsensectl lightbar {} {} {} {}",
            controller.lightbar_colour[0],
            controller.lightbar_colour[1],
            controller.lightbar_colour[2],
            controller.lightbar_colour[3]
        )
    } else {
        "dualsensectl lightbar off".to_string()
    };

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(_) => {
            controller.lightbar_enabled = state;
            info!(
                "Successfully executed lightbar toggle command. State: {}",
                if state { "On" } else { "Off" }
            );
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
        }
    }
}

pub fn change_playerleds_amount(state: u8, controller: &mut Controller) {
    if !(0..=5).contains(&state) {
        error!(
            "Invalid player LED state: {}. Must be between 0 and 5.",
            state
        );
        return;
    }

    let command = format!("dualsensectl player-leds {}", state);
    info!("Executing command: {}", command);

    if let Err(err) = Command::new("sh").arg("-c").arg(&command).output() {
        error!("Failed to execute command '{}': {}", command, err);
        return;
    }

    controller.playerleds = state;
}

pub fn change_lightbar_colour(state: Vec<u8>, controller: &mut Controller) {
    if state.len() != 4 {
        error!(
            "Invalid lightbar state: Expected 4 values (R, G, B, Brightness), got {}",
            state.len()
        );
        return;
    }

    let command = format!(
        "dualsensectl lightbar {} {} {} {}",
        state[0], state[1], state[2], state[3]
    );
    info!("Executing command: {}", command);

    if let Err(err) = Command::new("sh").arg("-c").arg(&command).output() {
        error!("Failed to execute command '{}': {}", command, err);
        return;
    }

    controller.lightbar_colour = state;
    controller.lightbar_enabled = true;
    info!("Lightbar colour changed and enabled.");
}

pub fn report_battery(controller: &mut Controller) -> String {
    let command = "dualsensectl battery";

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                info!("Command output: {}", stdout.trim());

                let battery_percentage = stdout
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(0);

                controller.battery_percentage = battery_percentage;

                format!("{}%", battery_percentage)
            } else {
                error!("Failed to parse command stdout as UTF-8");
                "Error: Invalid UTF-8 output".to_string()
            }
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
            format!("Error: {}", err)
        }
    }
}
