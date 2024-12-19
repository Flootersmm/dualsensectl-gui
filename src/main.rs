#![allow(dead_code)]

mod dualsensectl;
mod gui;
mod save;
mod structs;

use env_logger::Builder;
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use gui::ui::*;
use log::Level;
use save::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

const APP_ID: &str = "org.gtk_rs.Dualsensectl";

fn main() -> glib::ExitCode {
    let app_paths = Arc::new(AppPaths::new());
    let controller = Arc::new(Mutex::new(load_state(&app_paths)));

    truncate_log(&app_paths.log_file);
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&app_paths.log_file)
        .expect("Failed to open log file");

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
            )
            .unwrap();

            writeln!(buf, "{} {}: {}", timestamp, level, record.args())
        })
        .init();

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let window = build_ui(app, Arc::clone(&controller), Arc::clone(&app_paths));
        window.present();
    });

    app.run()
}
