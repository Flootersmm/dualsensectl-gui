use gtk::Button;
use gtk::Entry;
use gtk::LevelBar;
use gtk::{prelude::*, Label};
use gtk::{Box, InputPurpose, Orientation, Switch};

pub fn create_labeled_level_bar(
    label_text: &str,
    initial_value: f64,
    min: f64,
    max: f64,
) -> (Box, LevelBar) {
    // Create the label
    let label = Label::new(Some(label_text));

    // Create the level bar
    let level_bar = LevelBar::builder()
        .min_value(min)
        .max_value(max)
        .value(initial_value)
        .margin_top(6)
        .margin_bottom(12)
        .hexpand(true)
        .halign(gtk::Align::Center)
        .build();

    level_bar.set_width_request(200); // Ensure visibility with explicit width

    let box_with_label = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .build();

    box_with_label.append(&label);
    box_with_label.append(&level_bar);

    (box_with_label, level_bar)
}

pub fn create_labeled_entry(label_text: &str) -> (Box, Entry) {
    let label = Label::new(Some(label_text));
    let entry = Entry::builder()
        .margin_top(6)
        .margin_bottom(12)
        .hexpand(false)
        .halign(gtk::Align::Center)
        .input_purpose(InputPurpose::Number)
        .build();

    let box_with_label = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .build();

    box_with_label.append(&label);
    box_with_label.append(&entry);

    (box_with_label, entry)
}

pub fn create_labeled_button(label_text: &str) -> (Box, Button) {
    let label = Label::new(Some(label_text));
    let button = Button::builder()
        .label(label_text)
        .margin_top(6)
        .margin_bottom(12)
        .hexpand(false)
        .halign(gtk::Align::Center)
        .build();

    let box_with_label = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .build();

    box_with_label.append(&label);
    box_with_label.append(&button);

    (box_with_label, button)
}
pub fn create_labeled_switch(label_text: &str, active: bool) -> (Box, Switch) {
    let label = Label::new(Some(label_text));
    let switch = Switch::new();
    switch.set_margin_top(6);
    switch.set_margin_bottom(12);
    switch.set_active(active);
    switch.set_hexpand(false);
    switch.set_halign(gtk::Align::Center);

    let box_with_label = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .build();

    box_with_label.append(&label);
    box_with_label.append(&switch);

    (box_with_label, switch)
}

pub fn set_margins<W: gtk::prelude::WidgetExt>(widget: &W, margin: i32) {
    widget.set_margin_top(margin);
    widget.set_margin_bottom(margin);
    widget.set_margin_start(margin);
    widget.set_margin_end(margin);
}
