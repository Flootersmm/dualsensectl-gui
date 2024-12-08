mod custom_button;
mod dualsensectl;
mod file_ops;
mod gui;
mod save;

use env_logger::Builder;
use gui::ui::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.Dualsensectl";

fn main() -> glib::ExitCode {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs/dualsensectl.log")
        .unwrap();

    env::set_var("RUST_LOG", "info");
    Builder::from_default_env()
        .format(move |buf, record| {
            use chrono::Local;
            let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
            writeln!(
                log_file.try_clone().unwrap(),
                "{}: {}",
                timestamp,
                record.args()
            )?;
            // writing to stderr
            writeln!(buf, "{}: {}", timestamp, record.args())
        })
        .init();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let window = build_ui(app);
        window.present();
    });
    app.run()
}
