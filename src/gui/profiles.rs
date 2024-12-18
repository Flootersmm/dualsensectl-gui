use gtk::prelude::*;
use gtk::{Box, Label, Orientation};

pub fn create_profiles_page() -> Box {
    let profiles_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label = Label::new(Some("Profiles Page\nManage your profiles here."));
    label.set_halign(gtk::Align::Center);
    profiles_box.append(&label);

    profiles_box
}
