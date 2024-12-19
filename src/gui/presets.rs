use gtk::{prelude::*, ScrolledWindow};
use gtk::{Box, Label, Orientation, Separator};
use log::{error, info};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::structs::{Controller, Trigger, TriggerEffect};

pub struct Preset {
    pub name: &'static str,
    pub description: &'static str,
    pub command: &'static str,
    pub category: &'static str,
}

pub fn get_presets() -> Vec<Preset> {
    vec![
        Preset {
            name: "Right Trigger Constant Resistance",
            description: "Give a slight constant resistance to the right trigger from the beginning of the course.",
            command: "dualsensectl trigger right feedback 0 1",
            category: "Feedback Mode",
        },
        Preset {
            name: "Left Trigger End Resistance",
            description: "Give a slight constant resistance to the left trigger from the end of the course.",
            command: "dualsensectl trigger left feedback 9 1",
            category: "Feedback Mode",
        },
        Preset {
            name: "Both Triggers Strong Mid Resistance",
            description: "Give a strong constant resistance to both triggers midway.",
            command: "dualsensectl trigger both feedback 5 8",
            category: "Feedback Mode",
        },
        Preset {
            name: "Weapon Strong Start",
            description: "Strongly resist at the beginning of the course and stop resisting almost immediately.",
            command: "dualsensectl trigger both weapon 2 3 8",
            category: "Weapon Mode",
        },
        Preset {
            name: "Weapon Full Range",
            description: "Strongly resist at the beginning of the course and stop resisting at the end of the course.",
            command: "dualsensectl trigger both weapon 2 8 8",
            category: "Weapon Mode",
        },
        Preset {
            name: "Weapon Midway",
            description: "Slightly resist at the beginning of the course and stop resisting midway.",
            command: "dualsensectl trigger both weapon 2 5 1",
            category: "Weapon Mode",
        },
        Preset {
            name: "Bow Strong Snap",
            description: "Strongly resist at the beginning and snap back strongly when released.",
            command: "dualsensectl trigger both bow 1 8 8 8",
            category: "Bow Mode",
        },
        Preset {
            name: "Bow Soft Snap",
            description: "Strongly resist at the beginning and snap back slightly when released.",
            command: "dualsensectl trigger both bow 1 8 8 1",
            category: "Bow Mode",
        },
        Preset {
            name: "Galloping Heartbeat",
            description: "Feel a heartbeat whenever the trigger is pressed.",
            command: "dualsensectl trigger both galloping 0 9 1 3 1",
            category: "Galloping Mode",
        },
        Preset {
            name: "Galloping Horse",
            description: "Feel like a galloping horse whenever the trigger is pressed.",
            command: "dualsensectl trigger both galloping 0 9 2 3 2",
            category: "Galloping Mode",
        },
        Preset {
            name: "Machine Calm",
            description: "Vibrate strongly, calm down, and then return to vibrating strongly during the trigger course.",
            command: "dualsensectl trigger both machine 1 9 7 0 100 100",
            category: "Machine Mode",
        },
        Preset {
            name: "Machine Gun",
            description: "Feel like a machine gun during the entire trigger course.",
            command: "dualsensectl trigger both machine 1 9 7 7 9 1",
            category: "Machine Mode",
        },
        Preset {
            name: "Machine Gun Burst",
            description: "Feel bursts of machine gun vibrations during the trigger course.",
            command: "dualsensectl trigger both machine 1 9 7 0 18 12",
            category: "Machine Mode",
        },
        Preset {
            name: "Vibration 60Hz",
            description: "Slightly vibrate at 60Hz from the beginning of the course.",
            command: "dualsensectl trigger both vibration 1 1 60",
            category: "Vibration Mode",
        },
        Preset {
            name: "Vibration 120Hz",
            description: "Strongly vibrate at 120Hz from the beginning of the course.",
            command: "dualsensectl trigger both vibration 1 8 120",
            category: "Vibration Mode",
        },
        Preset {
            name: "Giant Clock",
            description: "Feel like you are holding a giant clock with vibrations at 1Hz.",
            command: "dualsensectl trigger both vibration 1 8 1",
            category: "Vibration Mode",
        },
        Preset {
            name: "Feedback Double Tap",
            description: "Resistance at the beginning, stops, and then resistance again midway.",
            command: "dualsensectl trigger both feedback-raw 1 2 8 0 0 1 2 8 0 0",
            category: "Feedback-raw Mode",
        },
        Preset {
            name: "Rusty Trigger",
            description: "Simulate a rusty trigger with varying resistances.",
            command: "dualsensectl trigger both feedback-raw 0 8 0 8 0 8 0 8 0 8",
            category: "Feedback-raw Mode",
        },
        Preset {
            name: "Feedback Harder Press",
            description: "The more you press, the harder the resistance, until the last position.",
            command: "dualsensectl trigger both feedback-raw 0 1 2 3 4 5 6 7 8 0",
            category: "Feedback-raw Mode",
        },
        Preset {
            name: "Vibration Increase",
            description: "The more you press, the more you feel the vibration at 100Hz.",
            command: "dualsensectl trigger both vibration-raw 0 1 2 3 4 5 6 7 8 8 100",
            category: "Vibration-raw Mode",
        },
        Preset {
            name: "Midway Vibration",
            description: "Feel vibration at 25Hz only when the trigger is around midway.",
            command: "dualsensectl trigger both vibration-raw 0 0 0 0 5 8 5 0 0 0 25",
            category: "Vibration-raw Mode",
        },
        Preset {
            name: "Disable Feedback",
            description: "Disable all vibrations and resistances.",
            command: "dualsensectl trigger both off",
            category: "Off Mode",
        },
    ]
}

pub fn run_command(command: &str, controller: &mut Controller) -> Result<(), String> {
    info!("Executing command: {}", command);

    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Command is empty".to_string());
    }

    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .map_err(|e| format!("Failed to execute command: {e}"))?;

    if output.status.success() {
        info!("Command succeeded: {}", command);

        if parts.len() > 2 && parts[0] == "dualsensectl" && parts[1] == "trigger" {
            let side = parts[2].to_string();
            match parts[3] {
                "feedback" if parts.len() >= 6 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Feedback {
                            position: parts[4].parse().unwrap_or(0),
                            strength: parts[5].parse().unwrap_or(0),
                        },
                    };
                }
                "weapon" if parts.len() >= 7 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Weapon {
                            start: parts[4].parse().unwrap_or(0),
                            stop: parts[5].parse().unwrap_or(0),
                            strength: parts[6].parse().unwrap_or(0),
                        },
                    };
                }
                "bow" if parts.len() >= 8 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Bow {
                            start: parts[4].parse().unwrap_or(0),
                            stop: parts[5].parse().unwrap_or(0),
                            strength: parts[6].parse().unwrap_or(0),
                            snapforce: parts[7].parse().unwrap_or(0),
                        },
                    };
                }
                "galloping" if parts.len() >= 8 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Galloping {
                            start: parts[4].parse().unwrap_or(0),
                            stop: parts[5].parse().unwrap_or(0),
                            first_foot: parts[6].parse().unwrap_or(0),
                            second_foot: parts[7].parse().unwrap_or(0),
                            frequency: parts[8].parse().unwrap_or(0),
                        },
                    };
                }
                "machine" if parts.len() >= 9 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Machine {
                            start: parts[4].parse().unwrap_or(0),
                            stop: parts[5].parse().unwrap_or(0),
                            strength_a: parts[6].parse().unwrap_or(0),
                            strength_b: parts[7].parse().unwrap_or(0),
                            frequency: parts[8].parse().unwrap_or(0),
                            period: parts[9].parse().unwrap_or(0),
                        },
                    };
                }
                "vibration" if parts.len() >= 7 => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Vibration {
                            position: parts[4].parse().unwrap_or(0),
                            amplitude: parts[5].parse().unwrap_or(0),
                            frequency: parts[6].parse().unwrap_or(0),
                        },
                    };
                }
                "feedback-raw" if parts.len() >= 14 => {
                    let mut strength = [0u8; 10];
                    for i in 0..10 {
                        strength[i] = parts[4 + i].parse().unwrap_or(0);
                    }
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::FeedbackRaw { strength },
                    };
                }
                "vibration-raw" if parts.len() >= 14 => {
                    let mut amplitude = [0u8; 10];
                    for i in 0..10 {
                        amplitude[i] = parts[4 + i].parse().unwrap_or(0);
                    }
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::VibrationRaw {
                            amplitude,
                            frequency: parts[14].parse().unwrap_or(0),
                        },
                    };
                }
                "off" => {
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Off,
                    };
                }
                "mode" => {
                    let params = parts[4..].iter().map(|s| (*s).to_string()).collect();
                    controller.trigger = Trigger {
                        side,
                        effect: TriggerEffect::Mode { params },
                    };
                }
                _ => error!("Unsupported trigger command: {}", command),
            }
        }

        Ok(())
    } else {
        error!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn apply_preset(preset: &Preset, controller: &mut Controller) {
    match run_command(preset.command, controller) {
        Ok(()) => info!("Preset '{}' applied successfully.", preset.name),
        Err(err) => error!("Failed to apply preset '{}': {}", preset.name, err),
    }
}

pub fn create_presets_page(controller: &Arc<Mutex<Controller>>) -> ScrolledWindow {
    let presets_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(400)
        .min_content_height(400)
        .child(&presets_box)
        .build();

    let mut current_category = "";

    for preset in get_presets() {
        if preset.category != current_category {
            current_category = preset.category;

            let category_label = Label::new(Some(current_category));
            category_label.set_halign(gtk::Align::Start);
            presets_box.append(&category_label);

            let separator = Separator::new(Orientation::Horizontal);
            presets_box.append(&separator);
        }

        let button = gtk::Button::with_label(preset.name);
        button.set_tooltip_text(Some(preset.description));

        let command = preset.command.to_string();
        let controller_clone = Arc::clone(controller);

        button.connect_clicked(move |_| {
            let controller_clone_inner = Arc::clone(&controller_clone);
            let command_clone = command.clone();

            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone_inner.lock() {
                    if let Err(err) = run_command(&command_clone, &mut ctrl) {
                        eprintln!("Failed to execute command '{command_clone}': {err}");
                    }
                } else {
                    eprintln!("Failed to lock controller for preset '{command_clone}'.");
                }
            });
        });

        presets_box.append(&button);
    }

    scrolled_window
}
