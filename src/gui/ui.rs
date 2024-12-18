use gtk::gdk;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::dualsensectl::*;
use crate::gui::utils::*;
use crate::save::*;
use crate::structs::{Controller, Speaker};

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Adjustment, Application, ApplicationWindow, Box, Button, ColorDialog, ColorDialogButton,
    DropDown, Grid, Label, Orientation, Scale, Switch,
};

// TODO: Also make .desktop

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
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let lightbar_switch = Switch::new();
    lightbar_switch.set_margin_top(6);
    lightbar_switch.set_margin_bottom(12);
    lightbar_switch.set_active(controller_state.lightbar_enabled);
    lightbar_switch.set_hexpand(false);
    lightbar_switch.set_halign(gtk::Align::Center);
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
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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
                if let Err(err) = save_state(&*ctrl) {
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
                if let Err(err) = save_state(&*ctrl) {
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

/// Creates and manages the player LED controls (dropdown).
fn create_playerleds_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Grid {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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
                    if let Err(err) = save_state(&*ctrl) {
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
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let speaker_item = gtk::StringList::new(&["Internal", "Headphones", "Both"]);

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
                2 => Speaker::Both,
                _ => Speaker::Internal,
            };

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    toggle_speaker(speaker, &mut ctrl);
                    if let Err(err) = save_state(&*ctrl) {
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
    let volume = volume_adjustment.value().round() as u8;
    volume_slider.connect_value_changed(move |_| {
        let controller_clone = Arc::clone(&controller_clone_2);
        thread::spawn(move || {
            if let Ok(mut ctrl) = controller_clone.lock() {
                change_volume(volume, &mut ctrl);
                if let Err(err) = save_state(&*ctrl) {
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
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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

            let mut attenuation = vec![0; 2];
            attenuation[0] = attenuation_rumble;

            ////////
            ////////
            // TODO: attenuation as Vec<u8> with 2 elements, use two dropdowns for the fields and
            // combine
            ////////

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    change_attenuation_amount(attenuation, &mut ctrl);
                    if let Err(err) = save_state(&*ctrl) {
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

            let mut attenuation = vec![0; 2];
            attenuation[1] = attenuation_trigger;

            ////////
            // TOOD: attenuations should be linked to struct
            ////////
            //
            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    change_attenuation_amount(attenuation, &mut ctrl);
                    if let Err(err) = save_state(&*ctrl) {
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
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let trigger_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5"]);

    let trigger_dropdown = DropDown::builder()
        .model(&trigger_items)
        .selected(1)
        .build();

    trigger_dropdown.connect_selected_notify({
        let controller = Arc::clone(&controller);
        let trigger_dropdown = trigger_dropdown.clone();
        move |_| {
            let trigger = trigger_dropdown.selected() as u8;

            let controller_clone = Arc::clone(&controller);
            thread::spawn(move || {
                if let Ok(mut ctrl) = controller_clone.lock() {
                    change_trigger(&mut ctrl);
                    if let Err(err) = save_state(&*ctrl) {
                        eprintln!("Failed to save controller state: {}", err);
                    }
                } else {
                    eprintln!("Failed to lock controller for player LED change.");
                }
            });
        }
    });

    grid.attach(&Label::new(Some("Triggers")), 0, 0, 1, 1);
    grid.attach(&trigger_dropdown, 1, 0, 1, 1);

    grid
}

//////////////////////////////////////////////////////////
// Main UI Function
//////////////////////////////////////////////////////////

pub fn build_ui(app: &Application, controller: Arc<Mutex<Controller>>) -> ApplicationWindow {
    let controller_state = load_state();

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

    let settings_grid = gtk::Grid::builder()
        .row_spacing(10)
        .column_spacing(20)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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

    let save_button = Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let refresh_button = Button::builder()
        .label("Refresh")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let optsbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    optsbox.append(&save_button);
    optsbox.append(&refresh_button);

    let box_main = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    box_main.append(&settings_grid);
    box_main.append(&optsbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dualsensectl GUI")
        .child(&box_main)
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
