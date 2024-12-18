use log::{error, info};
use std::process::Command;

use crate::structs::*;

/// Enables/disables the lightbar
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

/// Changes the player LED amount, 0-5
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

/// Changes the speaker output
///
/// Internal, Headphone, Monoheadphone (left side), Both (Internal &
/// Headphone)
pub fn toggle_speaker(state: Speaker, controller: &mut Controller) {
    let mut _command = "".to_string();

    match state {
        Speaker::Internal => _command = "dualsensectl speaker internal".to_string(),
        Speaker::Headphone => _command = "dualsensectl speaker headphone".to_string(),
        Speaker::Monoheadphone => _command = "dualsensectl speaker monoheadphone".to_string(),
        Speaker::Both => _command = "dualsensectl speaker both".to_string(),
    }

    info!("Executing command: {}", _command);

    if let Err(err) = Command::new("sh").arg("-c").arg(&_command).output() {
        error!("Failed to execute command '{}': {}", _command, err);
        return;
    }

    controller.speaker = state;
}

/// Changes the lightbar colour with RGB BRIGHTNESS, 0-255
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

/// Enables/disables the microphone
pub fn toggle_microphone(controller: &mut Controller) {
    let command = if controller.microphone {
        "dualsensectl microphone off".to_string()
    } else {
        "dualsensectl microphone on".to_string()
    };

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(_) => {
            controller.microphone = !controller.microphone;
            info!(
                "Successfully executed microphone toggle command. State: {}",
                if !controller.microphone { "On" } else { "Off" }
            );
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
        }
    }
}

/// Enables/disables the microphone LED
pub fn toggle_microphone_led(controller: &mut Controller) {
    let command = if controller.microphone_led {
        "dualsensectl microphone-led off".to_string()
    } else {
        "dualsensectl microphone-led on".to_string()
    };

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(_) => {
            controller.microphone_led = !controller.microphone_led;
            info!(
                "Successfully executed microphone-led toggle command. State: {}",
                if !controller.microphone_led {
                    "On"
                } else {
                    "Off"
                }
            );
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
        }
    }
}

/// Changes speaker volume, 0-255
///
/// 150+ is audible on Internal
///
/// 50+ is audible on Headphones
pub fn change_volume(volume: u8, controller: &mut Controller) {
    let command = format!("dualsensectl volume {}", volume);

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(_) => {
            controller.volume = volume;
            info!(
                "Successfully executed volume toggle command. State: {}",
                controller.volume
            );
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
        }
    }
}

/// Changes attenuation amount, (RUMBLE, TRIGGER) 0-7
pub fn change_attenuation_amount(attenuation: Vec<u8>, controller: &mut Controller) {
    if !(0..=7).contains(&attenuation[0]) || !(0..=7).contains(&attenuation[1]) {
        error!(
            "Invalid player attentuation attenuation: {} {}. RUMBLE and TRIGGER must be between 0 and 7.",
            attenuation[0], attenuation[1]
        );
        return;
    }

    let command = format!(
        "dualsensectl attenuation {} {}",
        attenuation[0], attenuation[1]
    );
    info!("Executing command: {}", command);

    if let Err(err) = Command::new("sh").arg("-c").arg(&command).output() {
        error!("Failed to execute command '{}': {}", command, err);
        return;
    }

    controller.attenuation = attenuation;
}

/// Changes trigger motor profile
pub fn change_triggers(trigger: &Trigger) {
    let command = format!("dualsensectl {}", trigger.to_command());

    info!("Executing command: {}", command);

    match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                info!("Command executed successfully: {}", stdout.trim());
            } else {
                error!("Failed to parse command stdout as UTF-8");
            }
        }
        Err(err) => {
            error!("Failed to execute command '{}': {}", command, err);
        }
    }
}

/// Reports battery level
///
/// Returns string 'u8%'
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
