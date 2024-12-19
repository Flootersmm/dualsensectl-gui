use std::collections::HashMap;

use gtk::Button;
use gtk::LevelBar;
use gtk::{prelude::*, Label};
use gtk::{Box, DropDown, Entry, Grid, InputPurpose, Orientation, Popover, Switch, Widget};

/// Trigger field min, max, and tooltip
#[derive(Clone)]
pub struct FieldConstraint {
    pub min: u8,
    pub max: u8,
    pub tooltip: String,
}

pub fn get_field_constraints() -> HashMap<String, FieldConstraint> {
    let mut constraints = HashMap::new();

    // Feedback Mode
    constraints.insert(
        "Feedback_Position".to_string(),
        FieldConstraint {
            min: 0,
            max: 9,
            tooltip: "Position must be between 0 and 9.".to_string(),
        },
    );
    constraints.insert(
        "Feedback_Strength".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Strength must be between 1 and 8.".to_string(),
        },
    );

    // Weapon Mode
    constraints.insert(
        "Weapon_Start".to_string(),
        FieldConstraint {
            min: 2,
            max: 7,
            tooltip: "Start position must be between 2 and 7.".to_string(),
        },
    );
    constraints.insert(
        "Weapon_Stop".to_string(),
        FieldConstraint {
            min: 3, // Dynamically updated based on "Start"
            max: 8,
            tooltip: "Stop position must be between Start+1 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Weapon_Strength".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Strength must be between 1 and 8.".to_string(),
        },
    );

    // Bow Mode
    constraints.insert(
        "Bow_Start".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Start position must be between 1 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Bow_Stop".to_string(),
        FieldConstraint {
            min: 2,
            max: 8,
            tooltip: "Stop position must be between Start+1 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Bow_Strength".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Strength must be between 1 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Bow_Snapforce".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Snapforce must be between 1 and 8.".to_string(),
        },
    );

    // Galloping Mode
    constraints.insert(
        "Galloping_Start".to_string(),
        FieldConstraint {
            min: 0,
            max: 8,
            tooltip: "Start position must be between 0 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Galloping_Stop".to_string(),
        FieldConstraint {
            min: 1,
            max: 9,
            tooltip: "Stop position must be between Start+1 and 9.".to_string(),
        },
    );
    constraints.insert(
        "Galloping_FirstFoot".to_string(),
        FieldConstraint {
            min: 0,
            max: 6,
            tooltip: "First Foot must be between 0 and 6.".to_string(),
        },
    );
    constraints.insert(
        "Galloping_SecondFoot".to_string(),
        FieldConstraint {
            min: 1,
            max: 7,
            tooltip: "Second Foot must be between FirstFoot+1 and 7.".to_string(),
        },
    );
    constraints.insert(
        "Galloping_Frequency".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Frequency must be greater than 0 and ideally lower than 8.".to_string(),
        },
    );

    // Machine Mode
    constraints.insert(
        "Machine_Start".to_string(),
        FieldConstraint {
            min: 1,
            max: 8,
            tooltip: "Start position must be between 1 and 8.".to_string(),
        },
    );
    constraints.insert(
        "Machine_Stop".to_string(),
        FieldConstraint {
            min: 2,
            max: 9,
            tooltip: "Stop position must be between Start+1 and 9.".to_string(),
        },
    );
    constraints.insert(
        "Machine_StrengthA".to_string(),
        FieldConstraint {
            min: 0,
            max: 7,
            tooltip: "Strength A must be between 0 and 7.".to_string(),
        },
    );
    constraints.insert(
        "Machine_StrengthB".to_string(),
        FieldConstraint {
            min: 0,
            max: 7,
            tooltip: "Strength B must be between 0 and 7.".to_string(),
        },
    );
    constraints.insert(
        "Machine_Frequency".to_string(),
        FieldConstraint {
            min: 1,
            max: 255,
            tooltip: "Frequency must be greater than 0.".to_string(),
        },
    );
    constraints.insert(
        "Machine_Period".to_string(),
        FieldConstraint {
            min: 0,
            max: 255,
            tooltip: "Period must be between 0 and 255.".to_string(),
        },
    );

    // FeedbackRaw Mode
    constraints.insert(
        "FeedbackRaw_Strength".to_string(),
        FieldConstraint {
            min: 0,
            max: 8,
            tooltip: "Strength values must be between 0 and 8, exactly 10 values.".to_string(),
        },
    );

    // VibrationRaw Mode
    constraints.insert(
        "VibrationRaw_Amplitude".to_string(),
        FieldConstraint {
            min: 0,
            max: 255,
            tooltip: "Amplitude values must be between 0 and 255, exactly 10 values.".to_string(),
        },
    );
    constraints.insert(
        "VibrationRaw_Frequency".to_string(),
        FieldConstraint {
            min: 1,
            max: 255,
            tooltip: "Frequency must be greater than 0 and up to 255.".to_string(),
        },
    );

    // Mode (Custom Params)
    constraints.insert(
        "Mode_Params".to_string(),
        FieldConstraint {
            min: 0,
            max: 255,
            tooltip: "Enter 9 comma-separated values (0-255).".to_string(),
        },
    );

    constraints
}

pub fn validate_input(entry: &Entry, constraint: &FieldConstraint, popover: &Popover) -> bool {
    let text = entry.text();
    let is_valid = text.split(',').all(|v| {
        v.trim()
            .parse::<u8>()
            .map_or(false, |n| n >= constraint.min && n <= constraint.max)
    });

    if !is_valid {
        entry.set_css_classes(&["error"]);
        show_error_popover(
            entry.upcast_ref::<Widget>(),
            constraint.tooltip.as_str(),
            popover,
        );
    } else {
        entry.set_css_classes(&[]);
        popover.set_visible(false);
    }

    is_valid
}

pub fn show_error_popover(widget: &Widget, message: &str, popover: &Popover) {
    let label = Label::new(Some(message));
    popover.set_child(Some(&label));
    popover.set_parent(widget);
    popover.popup();
}

pub fn create_validated_input_field(
    grid: &Grid,
    row: i32,
    label_text: &str,
    constraint: FieldConstraint,
) -> Entry {
    let label = Label::new(Some(label_text));
    label.set_halign(gtk::Align::Start);

    let entry = Entry::builder()
        .input_purpose(gtk::InputPurpose::Digits)
        .tooltip_text(&constraint.tooltip)
        .hexpand(true)
        .build();

    let popover = Popover::new();
    entry.set_text("0");

    entry.connect_changed({
        let constraint = constraint.clone();
        let popover = popover.clone();
        move |e| {
            validate_input(e, &constraint, &popover);
        }
    });

    grid.attach(&label, 0, row, 1, 1);
    grid.attach(&entry, 1, row, 2, 1);

    entry
}

pub fn create_labeled_level_bar(
    label_text: &str,
    initial_value: f64,
    min: f64,
    max: f64,
) -> (Box, LevelBar) {
    let label = Label::new(Some(label_text));

    let level_bar = LevelBar::builder()
        .min_value(min)
        .max_value(max)
        .value(initial_value)
        .margin_top(12)
        .margin_bottom(12)
        .hexpand(true)
        .vexpand(true)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .build();

    level_bar.set_width_request(200);

    let box_with_label = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
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

pub fn validate_comma_separated_input_up_to_9(input: &str) -> bool {
    let values: Vec<&str> = input.split(',').collect();
    if values.len() > 9 {
        return false;
    }

    values.iter().all(|v| v.trim().parse::<u8>().is_ok())
}

pub fn validate_comma_separated_input_exact_10(input: &str) -> bool {
    let values: Vec<&str> = input.split(',').collect();
    if values.len() != 10 {
        return false;
    }

    values.iter().all(|v| v.trim().parse::<u8>().is_ok())
}

pub fn get_input_values(grid: &Grid) -> Vec<u8> {
    let mut values = Vec::new();
    let mut child = grid.first_child();

    while let Some(widget) = child {
        if let Some(entry) = widget.downcast_ref::<Entry>() {
            if let Ok(value) = entry.text().parse::<u8>() {
                values.push(value);
            }
        }
        child = widget.next_sibling();
    }

    values
}

pub fn create_help_popup(grid: &Grid, help_text: &str, position: (i32, i32)) {
    let help_button = Button::builder()
        .label("?")
        .tooltip_text("Click for help")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let popover = Popover::builder()
        .child(&Label::new(Some(help_text)))
        .build();

    popover.set_parent(&help_button);

    help_button.connect_clicked(move |_| {
        popover.popup();
    });

    grid.attach(&help_button, position.0, position.1, 1, 1);
}

pub fn clear_grid(grid: &Grid) {
    let mut child = grid.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        grid.remove(&widget);
    }
}

pub fn create_input_fields(grid: &Grid, labels: &[&str]) {
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

pub fn set_margins<W: gtk::prelude::WidgetExt>(widget: &W, margin: i32) {
    widget.set_margin_top(margin);
    widget.set_margin_bottom(margin);
    widget.set_margin_start(margin);
    widget.set_margin_end(margin);
}
