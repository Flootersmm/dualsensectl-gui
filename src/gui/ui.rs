use gtk::gdk;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::dualsensectl::*;
use crate::gui::utils::*;
use crate::save::*;
use crate::structs::Controller;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{
    Adjustment, Application, ApplicationWindow, Box, Button, ColorDialog, ColorDialogButton,
    DropDown, Label, Orientation, Scale, Separator, Switch,
};

//////////////////////////////////////////////////////////
// Utility Functions
//////////////////////////////////////////////////////////

/// Creates and manages the lightbar controls (color and brightness).
fn create_lightbar_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Box {
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

    let color_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Fill)
        .build();
    color_box.append(&color_dialog_button);
    color_box.append(&brightness_slider);

    color_dialog_button.connect_rgba_notify({
        let apply_lightbar_changes = apply_lightbar_changes.clone();
        move |_| {
            apply_lightbar_changes();
        }
    });

    brightness_slider.connect_value_changed(move |_| {
        apply_lightbar_changes();
    });

    color_box
}

/// Creates and manages the player LED controls (dropdown).
fn create_playerleds_controls(
    controller: Arc<Mutex<Controller>>,
    controller_state: &Controller,
) -> Box {
    let playerleds_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5"]);

    let playerleds_dropdown = DropDown::builder()
        .model(&playerleds_items)
        .selected(controller_state.playerleds.into())
        .build();

    let playerleds_label = Label::new(Some("Player LEDs"));

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

    let playerleds_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .build();
    playerleds_box.append(&playerleds_label);
    playerleds_box.append(&playerleds_dropdown);

    playerleds_box
}

//////////////////////////////////////////////////////////
// Main UI Function
//////////////////////////////////////////////////////////

pub fn build_ui(app: &Application, controller: Arc<Mutex<Controller>>) -> ApplicationWindow {
    let controller_state = load_state();

    // let (lightbar_box, lightbar_switch) =
    // create_labeled_switch("Lightbar", controller_state.lightbar_enabled);

    let lightbar_switch_label = Label::new(Some("Lightbar"));
    let lightbar_switch = Switch::new();
    lightbar_switch.set_margin_top(6);
    lightbar_switch.set_margin_bottom(12);
    lightbar_switch.set_active(controller_state.lightbar_enabled);
    lightbar_switch.set_hexpand(false);
    lightbar_switch.set_halign(gtk::Align::Center);

    let lightbar_switch_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .build();
    lightbar_switch_box.append(&lightbar_switch_label);
    lightbar_switch_box.append(&lightbar_switch);

    let microphone_switch_label = Label::new(Some("Microphone"));
    let microphone_switch = Switch::new();
    microphone_switch.set_margin_top(6);
    microphone_switch.set_margin_bottom(12);
    microphone_switch.set_active(controller_state.microphone);
    microphone_switch.set_hexpand(false);
    microphone_switch.set_halign(gtk::Align::Center);

    let microphone_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .build();
    microphone_box.append(&microphone_switch_label);
    microphone_box.append(&microphone_switch);

    let microphone_led_switch_label = Label::new(Some("Microphone_led"));
    let microphone_led_switch = Switch::new();
    microphone_led_switch.set_margin_top(6);
    microphone_led_switch.set_margin_bottom(12);
    microphone_led_switch.set_active(controller_state.microphone_led);
    microphone_led_switch.set_hexpand(false);
    microphone_led_switch.set_halign(gtk::Align::Center);

    let microphone_led_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .build();
    microphone_box.append(&microphone_led_switch_label);
    microphone_box.append(&microphone_led_switch);

    let volume_label = Label::new(Some("Volume"));
    let volume_adjustment =
        Adjustment::new(controller_state.volume as f64, 0.0, 255.0, 1.0, 10.0, 0.0);

    let volume_slider = Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&volume_adjustment)
        .value_pos(gtk::PositionType::Right)
        .hexpand_set(true)
        .build();
    let volume_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();
    volume_box.append(&volume_label);
    volume_box.append(&volume_slider);

    let (battery_box, battery_level_bar) = create_labeled_level_bar(
        "Battery",
        controller_state.battery_percentage.into(),
        0.0,
        100.0,
    );

    let refresh_button = Button::builder()
        .label("Refresh")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    refresh_button.connect_clicked({
        let battery_level_bar_clone = battery_level_bar.clone();
        let controller = Arc::clone(&controller);
        move |_| {
            if let Ok(mut ctrl) = controller.lock() {
                let battery_percentage = report_battery(&mut ctrl)
                    .trim_end_matches('%')
                    .parse::<f64>()
                    .unwrap_or(0.0);
                battery_level_bar_clone.set_value(battery_percentage);
            } else {
                eprintln!("Failed to lock controller for reporting battery.");
            }
        }
    });

    lightbar_switch.connect_state_set({
        let controller = Arc::clone(&controller);
        move |_, state| {
            if let Ok(mut ctrl) = controller.lock() {
                toggle_lightbar(state, &mut ctrl);
            }
            Propagation::Proceed
        }
    });

    let playerleds_box = create_playerleds_controls(Arc::clone(&controller), &controller_state);
    let color_box = create_lightbar_controls(Arc::clone(&controller), &controller_state);

    let save_button = gtk::Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    save_button.connect_clicked({
        let controller_clone = Arc::clone(&controller);
        move |_| {
            if let Ok(ctrl) = controller_clone.lock() {
                if let Err(err) = save_state(&ctrl) {
                    eprintln!("Failed to save controller state: {}", err);
                }
            } else {
                eprintln!("Failed to lock controller for saving state.");
            }
        }
    });

    let box_topmost = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .vexpand(false)
        .build();
    box_topmost.append(&battery_box);

    let box_settings = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .homogeneous(true)
        .margin_top(1)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();
    box_settings.append(&lightbar_switch_box);
    box_settings.append(&playerleds_box);
    box_settings.append(&microphone_box);
    box_settings.append(&microphone_led_box);
    box_settings.append(&volume_box);

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

    let spacer = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .build();

    let separator_top = Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(6)
        .build();

    let separator_color = Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(6)
        .build();

    let separator_settings = Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(6)
        .build();

    let box_main = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    box_main.append(&separator_top);
    box_main.append(&box_settings);
    box_main.append(&separator_settings);
    box_main.append(&color_box);
    box_main.append(&separator_color);
    box_main.append(&spacer);
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
