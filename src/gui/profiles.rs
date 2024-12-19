use gtk::{prelude::*, ScrolledWindow};
use gtk::{Box, Label, Orientation, Separator};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::structs::{Controller, Trigger, TriggerEffect};

pub fn create_profiles_page(controller: &Arc<Mutex<Controller>>) -> ScrolledWindow {
    let presets_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(400)
        .min_content_height(400)
        .child(&presets_box)
        .build();

    scrolled_window
}
