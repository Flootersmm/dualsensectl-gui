mod custom_button;

use std::process::Command;

use custom_button::CustomButton;
use gtk::glib::Propagation;
use gtk::{glib, Application, ApplicationWindow, Box, Orientation};
use gtk::{prelude::*, Switch};

use chrono::prelude::*;

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    let switch = Switch::new();
    switch.set_active(true);
    switch.set_halign(gtk::Align::Center);
    switch.set_valign(gtk::Align::Center);
    switch.set_width_request(50);

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();

    vbox.append(&button);
    vbox.append(&switch);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    // Connect the close request signal to check the switch state
    let switch_clone = switch.clone();
    window.connect_close_request(move |win| {
        if switch_clone.is_active() {
            let current_time = Utc::now();
            let formatted_time = format!("test{}.txt", current_time.format("%Y-%m-%d_%H-%M-%S"));

            if let Err(err) = Command::new("sh")
                .arg("-c")
                .arg(format!("mv ./test.txt ./logs/{}", formatted_time))
                .output()
            {
                eprintln!("Failed to move file: {}", err);
            }
        }

        // Destroy the window
        win.close();
        Propagation::Proceed
    });

    // Present window
    window.present();
}
