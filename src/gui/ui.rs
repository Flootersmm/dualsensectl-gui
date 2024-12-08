use crate::dualsensectl::*;
use crate::gui::utils::*;
use crate::save::*;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::Button;
use gtk::{Application, ApplicationWindow, Box, DropDown, Orientation, Separator};

pub fn build_ui(app: &Application) -> ApplicationWindow {
    let app_state = load_state();

    let (lightbar_box, lightbar_switch) =
        create_labeled_switch("Lightbar", app_state.lightbar_enabled);

    let (battery_box, battery_level_bar) =
        create_labeled_level_bar("Battery", app_state.battery_percentage, 0.0, 100.0);

    let refresh_button = Button::builder()
        .label("Refresh")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let playerleds_items = gtk::StringList::new(&["0", "1", "2", "3", "4", "5"]);
    let playerleds_dropdown = DropDown::builder()
        .model(&playerleds_items)
        .selected(app_state.playerleds as u32)
        .build();

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

    let save_button = Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let battery_level_bar_clone = battery_level_bar.clone();
    refresh_button.connect_clicked(move |_| {
        let battery_percentage = report_battery()
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);
        battery_level_bar_clone.set_value(battery_percentage);
    });

    lightbar_switch.connect_state_set(move |_, state| {
        toggle_lightbar(state);
        Propagation::Proceed
    });

    submit_button.connect_clicked({
        let playerleds_dropdown = playerleds_dropdown.clone();
        let lightbar_switch = lightbar_switch.clone();
        move |_| {
            let playerleds = playerleds_dropdown.selected();
            let lightbar_state = lightbar_switch.is_active();
            change_playerleds(playerleds, lightbar_state);
        }
    });

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

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(gtk::Align::Center)
        .build();
    hbox.append(&lightbar_box);
    hbox.append(&battery_box);

    let optsbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let separator = Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(6)
        .build();

    optsbox.append(&separator);

    optsbox.append(&save_button);
    optsbox.append(&refresh_button);

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
    vbox.append(&optsbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dualsensectl GUI")
        .child(&vbox)
        .build();

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
