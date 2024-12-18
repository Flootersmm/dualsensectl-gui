#![allow(dead_code)]

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
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::sync::Arc;
use std::sync::Mutex;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.Dualsensectl";
const MAX_LOG_SIZE: usize = 1024 * 1024; // 1 MB

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

fn truncate_log(log_path: &std::path::Path) {
    if let Ok(mut file) = File::open(log_path) {
        let metadata = file.metadata().expect("Failed to get file metadata");
        if metadata.len() as usize > MAX_LOG_SIZE {
            println!(
                "Truncating log file as it exceeds the max size of {} bytes.",
                MAX_LOG_SIZE
            );

            let mut buffer = Vec::with_capacity(MAX_LOG_SIZE / 2);
            file.seek(std::io::SeekFrom::End(-(MAX_LOG_SIZE as i64) / 2))
                .expect("Failed to seek in log file");
            file.read_to_end(&mut buffer)
                .expect("Failed to read log file");

            let mut truncated_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(log_path)
                .expect("Failed to truncate log file");
            truncated_file
                .write_all(&buffer)
                .expect("Failed to write truncated log file");
        }
    }
}

fn main() -> glib::ExitCode {
    let controller = Arc::new(Mutex::new(load_state()));

    let log_file_path = get_log_path();
    truncate_log(&log_file_path);
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
