use gtk::{prelude::*, ScrolledWindow};
use gtk::{Box, Orientation};
use std::sync::{Arc, Mutex};

use crate::structs::{Controller};

pub fn create_profiles_page(controller: &Arc<Mutex<Controller>>) -> ScrolledWindow {
    let presets_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    

    ScrolledWindow::builder()
        .min_content_width(400)
        .min_content_height(400)
        .child(&presets_box)
        .build()
}
