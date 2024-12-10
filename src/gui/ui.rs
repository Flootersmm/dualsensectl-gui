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
    DropDown, Grid, Label, Orientation, Scale, Separator, Switch,
};

//////////////////////////////////////////////////////////
// Utility Functions
//////////////////////////////////////////////////////////

/// Creates and manages the lightbar controls (color and brightness).
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

    window
}
