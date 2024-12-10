#![allow(dead_code)]

mod custom_button;
mod dualsensectl;
mod file_ops;
mod gui;
mod save;
mod structs;

use env_logger::Builder;
use gui::ui::*;
use log::Level;
use save::load_state;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.Dualsensectl";

fn main() -> glib::ExitCode {
    let controller = Arc::new(Mutex::new(load_state()));

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
            let level = match record.level() {
                Level::Info => "[INFO]",
                Level::Error => "[ERROR]",
                Level::Warn => "[WARN]",
                Level::Debug => "[DEBUG]",
                Level::Trace => "[TRACE]",
            };

            writeln!(
                log_file.try_clone().unwrap(),
                "{} {}: {}",
                timestamp,
                level,
                record.args()
            )?;

            writeln!(buf, "{} {}: {}", timestamp, level, record.args())
        })
        .init();

    let app = Application::builder().application_id(APP_ID).build();

    let controller_clone = Arc::clone(&controller);
    app.connect_activate(move |app| {
        let window = build_ui(app, Arc::clone(&controller_clone));
        window.present();
    });

    app.run()
}
