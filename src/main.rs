#![allow(dead_code)]

mod custom_button;
mod dualsensectl;
mod gui;
mod save;
mod structs;

use dirs_next as dirs;
use env_logger::Builder;
use gui::ui::*;
use log::Level;
use save::load_state;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.Dualsensectl";

// ~/.local/share/dualsensectl-gui/logs/dualsensectl.log
fn get_log_path() -> std::path::PathBuf {
    let log_dir = dirs::data_local_dir()
        .expect("Failed to determine local data directory")
        .join("dualsensectl-gui/logs");

    if !log_dir.exists() {
        println!(
            "Attempting to create log directory at: {}",
            log_dir.display()
        );
        fs::create_dir_all(&log_dir).expect("Failed to create log directory");
    }

    let log_path = log_dir.join("dualsensectl.log");
    println!("Log path is: {}", log_path.display());
    log_path
}

fn main() -> glib::ExitCode {
    let controller = Arc::new(Mutex::new(load_state()));

    let log_file_path = get_log_path();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
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

    let controller_clone = Arc::clone(&controller);
    app.connect_activate(move |app| {
        let window = build_ui(app, Arc::clone(&controller_clone));
        window.present();
    });

    app.run()
}
