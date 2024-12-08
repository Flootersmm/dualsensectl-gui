use crate::custom_button::CustomButton;
use crate::dualsensectl::*;
use crate::gui::utils::*;
use crate::save::*;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::Button;
use gtk::{Application, ApplicationWindow, Box, Orientation};

pub fn build_ui(app: &Application) -> ApplicationWindow {
    // Load saved state
    let app_state = load_state();

    // Create the "Main" button
    let button = CustomButton::new();
    set_margins(&button, 12);

    // Create the "Lightbar" labeled switch with the saved state
    let (lightbar_box, lightbar_switch) =
        create_labeled_switch("Lightbar", app_state.lightbar_enabled);

    // Initialize the "Battery" labeled level bar with the saved state
    let (battery_box, battery_level_bar) =
        create_labeled_level_bar("Battery", app_state.battery_percentage, 0.0, 100.0);

    // Add a refresh button for the battery level
    let refresh_button = Button::builder()
        .label("Refresh")
        .margin_top(6)
        .margin_bottom(12)
        .build();
    battery_box.append(&refresh_button);

    // Clone relevant widgets for closures
    let lightbar_switch_clone = lightbar_switch.clone();
    let battery_level_bar_clone = battery_level_bar.clone();

    // Update battery level and save state on refresh
    refresh_button.connect_clicked(move |_| {
        let battery_percentage = report_battery()
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);
        battery_level_bar_clone.set_value(battery_percentage);

        // Save the updated state
        let new_state = AppState {
            lightbar_enabled: lightbar_switch_clone.is_active(),
            battery_percentage,
        };
        if let Err(err) = save_state(&new_state) {
            eprintln!("Failed to save state: {}", err);
        }
    });

    // Save state whenever the lightbar switch is toggled
    let battery_level_bar_clone = battery_level_bar.clone();
    lightbar_switch.connect_state_set(move |_, state| {
        toggle_lightbar(state);
        let new_state = AppState {
            lightbar_enabled: state,
            battery_percentage: battery_level_bar_clone.value(),
        };
        if let Err(err) = save_state(&new_state) {
            eprintln!("Failed to save state: {}", err);
        }
        Propagation::Proceed
    });

    // Create layout
    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(gtk::Align::Center)
        .build();
    hbox.append(&lightbar_box);
    hbox.append(&battery_box);

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    vbox.append(&hbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dualsensectl GUI")
        .child(&vbox)
        .build();

    // Save state on window close
    let battery_level_bar_clone = battery_level_bar.clone();
    let lightbar_switch_clone = lightbar_switch.clone();
    window.connect_close_request(move |win| {
        let final_state = AppState {
            lightbar_enabled: lightbar_switch_clone.is_active(),
            battery_percentage: battery_level_bar_clone.value(),
        };
        if let Err(err) = save_state(&final_state) {
            eprintln!("Failed to save state: {}", err);
        }
        win.close();
        Propagation::Proceed
    });

    window
}
