use gtk::gdk;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::dualsensectl::*;
use crate::gui::presets::*;
use crate::gui::profiles::*;
use crate::gui::utils::*;
use crate::save::*;
use crate::structs::*;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Adjustment, Application, ApplicationWindow, Box, Button, ColorDialog, ColorDialogButton,
    DropDown, Entry, Grid, Label, Orientation, Scale, StringList, Switch,
};

// TODO: Also make .desktop
// speaker monoheadphone option

//////////////////////////////////////////////////////////
// Utility Functions
//////////////////////////////////////////////////////////

fn create_lightbar_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let lightbar_switch = Switch::new();
    lightbar_switch.set_margin_top(6);
    lightbar_switch.set_margin_bottom(12);
    lightbar_switch.set_active(controller_state.lightbar_enabled);
    lightbar_switch.set_hexpand(false);
    lightbar_switch.set_halign(gtk::Align::Center);

    lightbar_switch.connect_state_set({
        let controller_clone = Arc::clone(&controller);
        move |_, state| {
            let controller_clone_inner = Arc::clone(&controller_clone);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone_inner.lock() {
                    toggle_lightbar(state, &mut ctrl);
                    if let Err(err) = save_state(&ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for lightbar toggle.");
                }
            });
            Propagation::Proceed
        }
    });

    let color_dialog = ColorDialog::builder().build();
    let color_dialog_button = ColorDialogButton::builder().build();
    color_dialog_button.set_dialog(&color_dialog);

    let rgba_color = gdk::RGBA::new(
        controller_state.lightbar_colour[0] as f32 / 255.0,
        controller_state.lightbar_colour[1] as f32 / 255.0,
        controller_state.lightbar_colour[2] as f32 / 255.0,
        1.0,
    );
    color_dialog_button.set_rgba(&rgba_color);

    let brightness_adjustment = Adjustment::new(
        controller_state.lightbar_colour[3] as f64,
        0.0,
        255.0,
        1.0,
        10.0,
        0.0,
    );

    let brightness_slider = Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&brightness_adjustment)
        .value_pos(gtk::PositionType::Right)
        .hexpand_set(true)
        .build();

    let apply_lightbar_changes = {
        let controller = Arc::clone(&controller);
        let color_dialog_button = color_dialog_button.clone();
        let brightness_adjustment = brightness_adjustment.clone();
        move || {
            let rgba = color_dialog_button.rgba();
            let red = (rgba.red() * 255.0).round() as u8;
            let green = (rgba.green() * 255.0).round() as u8;
            let blue = (rgba.blue() * 255.0).round() as u8;
            let brightness = brightness_adjustment.value().round() as u8;

            let state = vec![red, green, blue, brightness];

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    change_lightbar_colour(state, &mut ctrl);
                } else {
                    eprintln!("Failed to lock controller for lightbar color change.");
                }
            });
        }
    };

    color_dialog_button.connect_rgba_notify({
        let apply_lightbar_changes = apply_lightbar_changes.clone();
        move |_| {
            apply_lightbar_changes();
        }
    });

    brightness_slider.connect_value_changed(move |_| {
        apply_lightbar_changes();
    });

    grid.attach(
        &{
            let label = Label::new(Some("Enabled"));
            label.set_halign(gtk::Align::Start);
            label
        },
        0,
        0,
        1,
        1,
    );
    grid.attach(&lightbar_switch, 1, 0, 1, 1);
    grid.attach(
        &{
            let label = Label::new(Some("Lightbar Color"));
            label.set_halign(gtk::Align::Start);
            label
        },
        0,
        1,
        1,
        1,
    );
    grid.attach(&color_dialog_button, 1, 1, 1, 1);
    grid.attach(
        &{
            let label = Label::new(Some("Brightness"));
            label.set_halign(gtk::Align::Start);
            label
        },
        0,
        2,
        1,
        1,
    );
    grid.attach(&brightness_slider, 1, 2, 6, 1);

    grid
}

fn create_microphone_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let microphone_switch = Switch::new();
    microphone_switch.set_margin_top(6);
    microphone_switch.set_margin_bottom(12);
    microphone_switch.set_active(controller_state.microphone);
    microphone_switch.set_hexpand(false);
    microphone_switch.set_halign(gtk::Align::Center);

    let microphone_led_switch = Switch::new();
    microphone_led_switch.set_margin_top(6);
    microphone_led_switch.set_margin_bottom(12);
    microphone_led_switch.set_active(controller_state.microphone_led);
    microphone_led_switch.set_hexpand(false);
    microphone_led_switch.set_halign(gtk::Align::Center);

    let controller_clone_1 = Arc::clone(&controller);
    let controller_clone_2 = Arc::clone(&controller);

    microphone_switch.connect_state_set(move |_, _| {
        let controller_clone = Arc::clone(&controller_clone_1);
        thread::spawn(move || {
            if let Ok(mut ctrl) = controller_clone.lock() {
                toggle_microphone(&mut ctrl);
                if let Err(err) = save_state(&ctrl) {
                    eprintln!("Failed to save controller state: {}", err);
                }
            } else {
                eprintln!("Failed to lock controller for microphone toggle.");
            }
        });
        Propagation::Proceed
    });

    microphone_led_switch.connect_state_set(move |_, _| {
        let controller_clone = Arc::clone(&controller_clone_2);
        thread::spawn(move || {
            if let Ok(mut ctrl) = controller_clone.lock() {
                toggle_microphone_led(&mut ctrl);
                if let Err(err) = save_state(&ctrl) {
                    eprintln!("Failed to save controller state: {}", err);
                }
            } else {
                eprintln!("Failed to lock controller for microphone led toggle.");
            }
        });
        Propagation::Proceed
    });

    grid.attach(
        &{
            let label = Label::new(Some("Enabled"));
            label.set_halign(gtk::Align::Start);
            label
        },
        0,
        0,
        1,
        1,
    );
    grid.attach(&microphone_switch, 1, 0, 1, 1);
    grid.attach(
        &{
            let label = Label::new(Some("Microphone LED"));
            label.set_halign(gtk::Align::Start);
            label
        },
        0,
        1,
        1,
        1,
    );
    grid.attach(&microphone_led_switch, 1, 1, 1, 1);

    grid
}

fn create_playerleds_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let playerleds_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5"]);

    let playerleds_dropdown = DropDown::builder()
        .model(&playerleds_items)
        .selected(controller_state.playerleds.into())
        .build();

    playerleds_dropdown.connect_selected_notify({
        let controller = Arc::clone(&controller);
        let playerleds_dropdown = playerleds_dropdown.clone();
        move |_| {
            let playerleds = playerleds_dropdown.selected() as u8;

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    change_playerleds_amount(playerleds, &mut ctrl);
                    if let Err(err) = save_state(&ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for player LED change.");
                }
            });
        }
    });

    grid.attach(&Label::new(Some("Player LEDs")), 0, 0, 1, 1);
    grid.attach(&playerleds_dropdown, 1, 0, 1, 1);

    grid
}

fn create_speaker_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let speaker_item = gtk::StringList::new(&["Internal", "Headphones", "Mono Headphones", "Both"]);

    let speaker_dropdown = DropDown::builder()
        .model(&speaker_item)
        .selected(controller_state.playerleds.into())
        .build();

    speaker_dropdown.connect_selected_notify({
        let controller = Arc::clone(&controller);
        let speaker_dropdown = speaker_dropdown.clone();
        move |_| {
            let speaker = match speaker_dropdown.selected() {
                0 => Speaker::Internal,
                1 => Speaker::Headphone,
                2 => Speaker::Monoheadphone,
                3 => Speaker::Both,
                _ => Speaker::Internal,
            };

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    toggle_speaker(speaker, &mut ctrl);
                    if let Err(err) = save_state(&ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for speaker change.");
                }
            });
        }
    });

    let volume_adjustment = Adjustment::new(
        controller_state.lightbar_colour[3] as f64,
        0.0,
        255.0,
        1.0,
        10.0,
        0.0,
    );

    let volume_slider = Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&volume_adjustment)
        .value_pos(gtk::PositionType::Right)
        .hexpand_set(true)
        .build();

    let controller_clone_2 = Arc::clone(&controller);

    volume_slider.connect_value_changed(move |_| {
        let controller_clone = Arc::clone(&controller_clone_2);
        let volume = volume_adjustment.value().round() as u8;

        thread::spawn(move || {
            if let Ok(mut ctrl) = controller_clone.lock() {
                change_volume(volume, &mut ctrl);
                if let Err(err) = save_state(&ctrl) {
                    eprintln!("Failed to save controller state: {}", err);
                }
            } else {
                eprintln!("Failed to lock controller for volume change.");
            }
        });
    });

    grid.attach(&speaker_dropdown, 0, 0, 1, 1);
    grid.attach(&volume_slider, 1, 0, 6, 1);

    grid
}

fn create_attenuation_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let attenuation_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5", "6", "7"]);

    let attenuation_rumble_dropdown = DropDown::builder()
        .model(&attenuation_items)
        .selected(controller_state.attenuation[0].into())
        .build();

    let attenuation_trigger_dropdown = DropDown::builder()
        .model(&attenuation_items)
        .selected(controller_state.attenuation[1].into())
        .build();

    attenuation_rumble_dropdown.connect_selected_notify({
        let controller = Arc::clone(&controller);
        let attenuation_rumble_dropdown = attenuation_rumble_dropdown.clone();
        move |_| {
            let attenuation_rumble = attenuation_rumble_dropdown.selected() as u8;

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    ctrl.attenuation[0] = attenuation_rumble;
                    change_attenuation_amount(ctrl.attenuation.clone(), &mut ctrl);
                    if let Err(err) = save_state(&ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for attenuation RUMBLE change.");
                }
            });
        }
    });

    attenuation_trigger_dropdown.connect_selected_notify({
        let controller = Arc::clone(&controller);
        let attenuation_trigger_dropdown = attenuation_trigger_dropdown.clone();
        move |_| {
            let attenuation_trigger = attenuation_trigger_dropdown.selected() as u8;

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    ctrl.attenuation[1] = attenuation_trigger;
                    change_attenuation_amount(ctrl.attenuation.clone(), &mut ctrl);
                    if let Err(err) = save_state(&ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for attenuation TRIGGER change.");
                }
            });
        }
    });

    grid.attach(&attenuation_rumble_dropdown, 1, 0, 1, 1);
    grid.attach(&attenuation_trigger_dropdown, 2, 0, 1, 1);

    grid
}

fn create_trigger_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = Grid::builder()
        .column_homogeneous(true)
        .row_spacing(6)
        .column_spacing(10)
        .build();
    set_margins(&grid, 12);

    let trigger_effects = Arc::new(vec![
        "Off",
        "Feedback",
        "Weapon",
        "Bow",
        "Galloping",
        "Machine",
        "Vibration",
        "FeedbackRaw",
        "VibrationRaw",
        "Mode",
    ]);

    let string_list = StringList::new(&trigger_effects);
    let effect_dropdown = DropDown::builder().model(&string_list).selected(0).build();

    let initial_index = trigger_effects
        .iter()
        .position(|e| e == &format!("{:?}", controller_state.trigger.effect))
        .unwrap_or(0);
    effect_dropdown.set_selected(initial_index as u32);

    let input_grid = Grid::builder().row_spacing(6).column_spacing(10).build();

    let apply_button = Button::builder()
        .label("Apply")
        .halign(gtk::Align::End)
        .build();

    create_help_popup(
        &grid,
        "Trigger Modes:\n\
         - Feedback: Position (0-9), Strength (1-8)\n\
         - Weapon: Start (2-7), Stop (Start+1 to 8), Strength (1-8)\n\
         - Bow: Start (1-8), Stop (Start+1 to 8), Strength (1-8), Snapforce (1-8)\n\
         - Galloping: Start (0-8), Stop (Start+1 to 9), First Foot (0-6), Second Foot (First Foot+1 to 7), Frequency (>0)\n\
         - Machine: Start (1-8), Stop (Start+1 to 9), Strength A/B (0-7), Frequency (>0), Period\n\
         - Vibration: Position (0-9), Amplitude (1-8), Frequency (>0)\n\
         - FeedbackRaw: 10 Strength values (0-8)\n\
         - VibrationRaw: 10 Amplitudes (0-255), Frequency\n\
         - Mode: 9 Params (comma-separated values)",
        (3, 0),
    );

    let constraints = get_field_constraints();

    effect_dropdown.connect_selected_notify({
        let input_grid = input_grid.clone();
        let trigger_effects = Arc::clone(&trigger_effects);

        move |dropdown| {
            clear_grid(&input_grid);
            let selected = dropdown.selected() as usize;

            match trigger_effects[selected] {
                "Feedback" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Position",
                        constraints["Feedback_Position"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        1,
                        "Strength",
                        constraints["Feedback_Strength"].clone(),
                    );
                }
                "Weapon" => {
                    let start_entry = create_validated_input_field(
                        &input_grid,
                        0,
                        "Start",
                        constraints["Weapon_Start"].clone(),
                    );
                    let stop_entry = create_validated_input_field(
                        &input_grid,
                        1,
                        "Stop",
                        constraints["Weapon_Stop"].clone(),
                    );

                    start_entry.connect_changed({
                        let stop_entry = stop_entry.clone();
                        move |e| {
                            if let Ok(start) = e.text().parse::<u8>() {
                                let dynamic_constraint = FieldConstraint {
                                    min: start + 1,
                                    max: 8,
                                    tooltip: format!("Stop must be between {} and 8", start + 1),
                                };
                                stop_entry.set_tooltip_text(Some(&dynamic_constraint.tooltip));
                            }
                        }
                    });

                    create_validated_input_field(
                        &input_grid,
                        2,
                        "Strength",
                        constraints["Weapon_Strength"].clone(),
                    );
                }
                "Bow" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Start",
                        constraints["Bow_Start"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        1,
                        "Stop",
                        constraints["Bow_Stop"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        2,
                        "Strength",
                        constraints["Bow_Strength"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        3,
                        "Snapforce",
                        constraints["Bow_Snapforce"].clone(),
                    );
                }
                "Galloping" => {
                    let start_entry = create_validated_input_field(
                        &input_grid,
                        0,
                        "Start",
                        constraints["Galloping_Start"].clone(),
                    );
                    let stop_entry = create_validated_input_field(
                        &input_grid,
                        1,
                        "Stop",
                        constraints["Galloping_Stop"].clone(),
                    );

                    start_entry.connect_changed({
                        let stop_entry = stop_entry.clone();
                        move |e| {
                            if let Ok(start) = e.text().parse::<u8>() {
                                let dynamic_constraint = FieldConstraint {
                                    min: start + 1,
                                    max: 9,
                                    tooltip: format!("Stop must be between {} and 9", start + 1),
                                };
                                stop_entry.set_tooltip_text(Some(&dynamic_constraint.tooltip));
                            }
                        }
                    });

                    let first_foot_entry = create_validated_input_field(
                        &input_grid,
                        2,
                        "First Foot",
                        constraints["Galloping_FirstFoot"].clone(),
                    );
                    let second_foot_entry = create_validated_input_field(
                        &input_grid,
                        3,
                        "Second Foot",
                        constraints["Galloping_SecondFoot"].clone(),
                    );

                    first_foot_entry.connect_changed({
                        let second_foot_entry = second_foot_entry.clone();
                        move |e| {
                            if let Ok(first_foot) = e.text().parse::<u8>() {
                                let dynamic_constraint = FieldConstraint {
                                    min: first_foot + 1,
                                    max: 7,
                                    tooltip: format!(
                                        "Second Foot must be between {} and 7",
                                        first_foot + 1
                                    ),
                                };
                                second_foot_entry
                                    .set_tooltip_text(Some(&dynamic_constraint.tooltip));
                            }
                        }
                    });

                    create_validated_input_field(
                        &input_grid,
                        4,
                        "Frequency",
                        constraints["Galloping_Frequency"].clone(),
                    );
                }
                "Machine" => {
                    let start_entry = create_validated_input_field(
                        &input_grid,
                        0,
                        "Start",
                        constraints["Machine_Start"].clone(),
                    );
                    let stop_entry = create_validated_input_field(
                        &input_grid,
                        1,
                        "Stop",
                        constraints["Machine_Stop"].clone(),
                    );

                    start_entry.connect_changed({
                        let stop_entry = stop_entry.clone();
                        move |e| {
                            if let Ok(start) = e.text().parse::<u8>() {
                                let dynamic_constraint = FieldConstraint {
                                    min: start + 1,
                                    max: 9,
                                    tooltip: format!("Stop must be between {} and 9", start + 1),
                                };
                                stop_entry.set_tooltip_text(Some(&dynamic_constraint.tooltip));
                            }
                        }
                    });

                    create_validated_input_field(
                        &input_grid,
                        2,
                        "Strength A",
                        constraints["Machine_StrengthA"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        3,
                        "Strength B",
                        constraints["Machine_StrengthB"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        4,
                        "Frequency",
                        constraints["Machine_Frequency"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        5,
                        "Period",
                        constraints["Machine_Period"].clone(),
                    );
                }
                "Vibration" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Position",
                        constraints["Feedback_Position"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        1,
                        "Amplitude",
                        constraints["FeedbackRaw_Strength"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        2,
                        "Frequency",
                        constraints["VibrationRaw_Frequency"].clone(),
                    );
                }
                "FeedbackRaw" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Strength [10]",
                        constraints["FeedbackRaw_Strength"].clone(),
                    );
                }
                "VibrationRaw" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Amplitude [10]",
                        constraints["VibrationRaw_Amplitude"].clone(),
                    );
                    create_validated_input_field(
                        &input_grid,
                        1,
                        "Frequency",
                        constraints["VibrationRaw_Frequency"].clone(),
                    );
                }
                "Mode" => {
                    create_validated_input_field(
                        &input_grid,
                        0,
                        "Params",
                        constraints["Mode_Params"].clone(),
                    );
                }
                _ => {}
            }
        }
    });

    apply_button.connect_clicked({
        let controller_clone = Arc::clone(&controller);
        let effect_dropdown = effect_dropdown.clone();
        let input_grid = input_grid.clone();
        let trigger_effects = Arc::clone(&trigger_effects);

        move |_| {
            let selected = effect_dropdown.selected() as usize;
            let mut new_effect = TriggerEffect::Off;

            match trigger_effects[selected] {
                "Feedback" => {
                    let values = get_input_values(&input_grid);
                    new_effect = TriggerEffect::Feedback {
                        position: values.first().cloned().unwrap_or_default(),
                        strength: values.get(1).cloned().unwrap_or_default(),
                    };
                }
                "Weapon" => {
                    let values = get_input_values(&input_grid);
                    new_effect = TriggerEffect::Weapon {
                        start: values.first().cloned().unwrap_or_default(),
                        stop: values.get(1).cloned().unwrap_or_default(),
                        strength: values.get(2).cloned().unwrap_or_default(),
                    };
                }
                _ => {}
            }

            if let Ok(mut ctrl) = controller_clone.lock() {
                ctrl.trigger.effect = new_effect;
                println!("Updated trigger: {:?}", ctrl.trigger);
                change_triggers(&ctrl.trigger);
            }
        }
    });

    grid.attach(&Label::new(Some("Trigger Effect:")), 0, 0, 1, 1);
    grid.attach(&effect_dropdown, 1, 0, 2, 1);
    grid.attach(&input_grid, 0, 1, 3, 1);
    grid.attach(&apply_button, 2, 2, 1, 1);

    grid
}

fn clear_grid(grid: &Grid) {
    let mut child = grid.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        grid.remove(&widget);
    }
}

fn create_input_fields(grid: &Grid, labels: &[&str]) {
    for (i, &label) in labels.iter().enumerate() {
        if label == "Params [1..=9]" {
            let side_dropdown = DropDown::builder()
                .model(&gtk::StringList::new(&["left", "right", "both"]))
                .selected(2)
                .build();

            let params_entry = Entry::builder()
                .input_purpose(gtk::InputPurpose::Digits)
                .tooltip_text(
                    "Enter up to 9 comma-separated values (0-255), e.g., 255,127,6,0,0,20,0,0,0",
                )
                .hexpand(true)
                .build();

            params_entry.set_text("0,0,0,0,0,0,0,0,0");

            params_entry.connect_changed(|entry| {
                let text = entry.text();
                let is_valid = validate_comma_separated_input_up_to_9(&text);
                if is_valid {
                    entry.set_css_classes(&[]);
                } else {
                    entry.set_css_classes(&["error"]);
                }
            });

            grid.attach(&Label::new(Some("Side")), 0, i as i32, 1, 1);
            grid.attach(&side_dropdown, 1, i as i32, 1, 1);
            grid.attach(&Label::new(Some(label)), 0, (i + 1) as i32, 1, 1);
            grid.attach(&params_entry, 1, (i + 1) as i32, 2, 1);
        } else if label.contains("[10]") {
            let entry = Entry::builder()
                .input_purpose(gtk::InputPurpose::Digits)
                .tooltip_text("Enter exactly 10 comma-separated values (0-255), e.g., 0,0,215,0,0,0,125,10,0,0")
                .hexpand(true)
                .build();

            entry.set_text("0,0,0,0,0,0,0,0,0,0");

            entry.connect_changed(|entry| {
                let text = entry.text();
                let is_valid = validate_comma_separated_input_exact_10(&text);
                if is_valid {
                    entry.set_css_classes(&[]);
                } else {
                    entry.set_css_classes(&["error"]);
                }
            });

            grid.attach(&Label::new(Some(label)), 0, i as i32, 1, 1);
            grid.attach(&entry, 1, i as i32, 2, 1);
        } else {
            let entry = Entry::builder()
                .input_purpose(gtk::InputPurpose::Digits)
                .hexpand(true)
                .build();

            entry.set_text("0");
            grid.attach(&Label::new(Some(label)), 0, i as i32, 1, 1);
            grid.attach(&entry, 1, i as i32, 2, 1);
        }
    }

    grid.set_visible(true);
}

//////////////////////////////////////////////////////////
// Main UI Function
//////////////////////////////////////////////////////////

pub fn build_ui(app: &Application, controller: Arc<Mutex<Controller>>) -> ApplicationWindow {
    let controller_state = load_state();

    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::SlideLeftRight)
        .transition_duration(300)
        .build();

    let stack_switcher = gtk::StackSwitcher::builder().stack(&stack).build();
    set_margins(&stack_switcher, 12);

    let main_controls_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    set_margins(&main_controls_box, 12);

    let battery_box = create_labeled_level_bar(
        "Battery",
        controller_state.battery_percentage.into(),
        0.0,
        100.0,
    )
    .0;

    let lightbar_controls_grid =
        create_lightbar_controls(Arc::clone(&controller), &controller_state);
    let playerleds_controls_grid =
        create_playerleds_controls(Arc::clone(&controller), &controller_state);
    let microphone_controls_grid =
        create_microphone_controls(Arc::clone(&controller), &controller_state);
    let speaker_controls_grid = create_speaker_controls(Arc::clone(&controller), &controller_state);
    let attenuation_controls_grid =
        create_attenuation_controls(Arc::clone(&controller), &controller_state);
    let trigger_controls_grid = create_trigger_controls(Arc::clone(&controller), &controller_state);

    let settings_grid = gtk::Grid::builder()
        .row_spacing(10)
        .column_spacing(20)
        .build();
    set_margins(&settings_grid, 12);

    settings_grid.attach(&battery_box, 0, 0, 1, 1);
    settings_grid.attach(&Label::new(Some("Lightbar Controls")), 0, 1, 2, 1);
    settings_grid.attach(&lightbar_controls_grid, 0, 3, 2, 1);
    settings_grid.attach(&Label::new(Some("Player LEDs")), 0, 4, 2, 1);
    settings_grid.attach(&playerleds_controls_grid, 0, 5, 2, 1);
    settings_grid.attach(&Label::new(Some("Microphone")), 0, 6, 2, 1);
    settings_grid.attach(&microphone_controls_grid, 0, 7, 2, 1);
    settings_grid.attach(&Label::new(Some("Speaker")), 0, 8, 2, 1);
    settings_grid.attach(&speaker_controls_grid, 0, 9, 2, 1);
    settings_grid.attach(&Label::new(Some("Attenuation")), 0, 10, 2, 1);
    settings_grid.attach(&attenuation_controls_grid, 0, 11, 2, 1);
    settings_grid.attach(&Label::new(Some("Triggers")), 0, 12, 2, 1);
    settings_grid.attach(&trigger_controls_grid, 0, 13, 2, 1);

    main_controls_box.append(&settings_grid);

    stack.add_titled(&main_controls_box, Some("main"), "Settings");

    let presets_page = create_presets_page(&Arc::clone(&controller));
    stack.add_titled(&presets_page, Some("presets"), "Presets");

    let profiles_page = create_profiles_page();
    stack.add_titled(&profiles_page, Some("profiles"), "Profiles");

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    set_margins(&main_box, 12);
    main_box.append(&stack_switcher);
    main_box.append(&stack);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dualsensectl GUI")
        .child(&main_box)
        .default_width(600)
        .default_height(400)
        .build();

    window.connect_close_request({
        let controller_clone = Arc::clone(&controller);
        move |win| {
            if let Ok(controller) = controller_clone.lock() {
                if let Err(err) = save_state(&controller) {
                    eprintln!("Failed to save controller state: {}", err);
                }
            } else {
                eprintln!("Failed to lock controller for saving state.");
            }
            win.close();
            Propagation::Proceed
        }
    });

    window
}
