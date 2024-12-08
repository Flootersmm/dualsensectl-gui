use crate::custom_button::CustomButton;
use crate::dualsensectl::*;
use crate::gui::utils::*;
use crate::save::*;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::Button;
use gtk::{Application, ApplicationWindow, Box, DropDown, Orientation};

pub fn build_ui(app: &Application) -> ApplicationWindow {
    // Load saved state
    let app_state = load_state();

    // Create labeled switch for lightbar
    let (lightbar_box, lightbar_switch) =
        create_labeled_switch("Lightbar", app_state.lightbar_enabled);

    // Create labeled level bar for battery
    let (battery_box, battery_level_bar) =
        create_labeled_level_bar("Battery", app_state.battery_percentage, 0.0, 100.0);

    // Create refresh button for battery
    let refresh_button = Button::builder()
        .label("Refresh")
        .margin_top(6)
        .margin_bottom(12)
        .build();
    battery_box.append(&refresh_button);

    // Create a dropdown for player LEDs
    let playerleds_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5"]);
    let playerleds_dropdown = DropDown::builder()
        .model(&playerleds_items)
        .selected(app_state.playerleds as u32)
        .build();

    // Create a box for the dropdown and submit button
    let submit_button = Button::builder()
        .label("Set Player LEDs")
        .margin_top(6)
        .margin_bottom(12)
        .build();

    let playerleds_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .build();
    playerleds_box.append(&playerleds_dropdown);
    playerleds_box.append(&submit_button);

    // Add save button at the bottom right
    let save_button = Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::End)
        .build();

    // Refresh button logic
    let battery_level_bar_clone = battery_level_bar.clone();
    refresh_button.connect_clicked(move |_| {
        let battery_percentage = report_battery()
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);
        battery_level_bar_clone.set_value(battery_percentage);
    });

    // Submit button logic for player LEDs
    submit_button.connect_clicked({
        let playerleds_dropdown = playerleds_dropdown.clone();
        move |_| {
            let playerleds = playerleds_dropdown.selected();
            change_playerleds(playerleds);
        }
    });

    // Save button logic
    save_button.connect_clicked({
        let battery_level_bar = battery_level_bar.clone();
        let lightbar_switch = lightbar_switch.clone();
        let playerleds_dropdown = playerleds_dropdown.clone();
        move |_| {
            let new_state = AppState {
                lightbar_enabled: lightbar_switch.is_active(),
                battery_percentage: battery_level_bar.value(),
                playerleds: playerleds_dropdown.selected(),
            };
            if let Err(err) = save_state(&new_state) {
                eprintln!("Failed to save state: {}", err);
            }
        }
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
    vbox.append(&playerleds_box);
    vbox.append(&save_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dualsensectl GUI")
        .child(&vbox)
        .build();

    // Save state on close
    let battery_level_bar_clone = battery_level_bar.clone();
    let lightbar_switch_clone = lightbar_switch.clone();
    let playerleds_dropdown_clone = playerleds_dropdown.clone();
    window.connect_close_request(move |win| {
        let final_state = AppState {
            lightbar_enabled: lightbar_switch_clone.is_active(),
            battery_percentage: battery_level_bar_clone.value(),
            playerleds: playerleds_dropdown_clone.selected(),
        };
        if let Err(err) = save_state(&final_state) {
            eprintln!("Failed to save state: {}", err);
        }
        win.close();
        Propagation::Proceed
    });

    window
}
